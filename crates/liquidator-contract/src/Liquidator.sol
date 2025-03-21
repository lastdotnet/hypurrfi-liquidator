// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Owned} from "solmate/auth/Owned.sol";
import {ERC20} from "solmate/tokens/ERC20.sol";
import {IPool} from "./interfaces/IPool.sol";
import {IUniswapV3SwapCallback} from "./interfaces/IUniswapV3SwapCallback.sol";
import {IUniswapV3PoolActions} from "./interfaces/IUniswapV3PoolActions.sol";
import {PoolAddress} from "./lib/PoolAddress.sol";
import {Path} from "./lib/Path.sol";
import {IKittenPair} from "./interfaces/IKittenPair.sol";
import {IKittenPairFactory} from "./interfaces/IKittenPairFactory.sol";
import {KittenswapLib} from "./lib/KittenswapLib.sol";
import {IKittenswapSwapCallback} from "./interfaces/IKittenswapSwapCallback.sol";

uint160 constant MIN_SQRT_RATIO = 4295128739;
/// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
uint160 constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970342;

contract Liquidator is Owned(msg.sender), IKittenswapSwapCallback, IUniswapV3SwapCallback {
    event LiquidatorAdded(address indexed liquidator);
    event LiquidatorRemoved(address indexed liquidator);
    struct SwapCallbackData {
        bytes path;
        address collateralAsset;
        address debtAsset;
        address user;
        uint256 debtToCover;
        uint256 amountToPay;
    }

    address public constant uniswapV3Factory = 0xB1c0fa0B789320044A6F623cFe5eBda9562602E3;
    address public constant kittenPairFactory = 0xDa12F450580A4cc485C3b501BAB7b0B3cbc3B31B;
    IPool public constant pool = IPool(0xceCcE0EB9DD2Ef7996e01e25DD70e461F918A14b);

    IKittenPair private activeKittenPair;

    mapping(address => bool) public isLiquidator;

    constructor() {}

    modifier onlyOwnerOrLiquidator() {
        require(msg.sender == owner || isLiquidator[msg.sender], "Only owner or liquidator can call this function");
        _;
    }

    function addLiquidator(address _liquidator) external onlyOwner {
        isLiquidator[_liquidator] = true;
        emit LiquidatorAdded(_liquidator);
    }

    function removeLiquidator(address _liquidator) external onlyOwner {
        isLiquidator[_liquidator] = false;
        emit LiquidatorRemoved(_liquidator);
    }

    /// @notice Performs a liquidation using a flash swap
    /// @param collateralAsset address of the collateral asset to be liquidated
    /// @param debtAsset address of the debt asset to be repaid
    /// @param user address of the user to be liquidated
    /// @param debtToCover amount of debt asset to repay in exchange for collateral
    /// @param swapPath encoded path of pools to swap collateral through, see: https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps
    /// @param swapVenue venue of the swap, either "kittenswap" or "hyperswap"
    function liquidate(
        address collateralAsset,
        address debtAsset,
        address user,
        uint256 debtToCover,
        bytes calldata swapPath,
        string calldata swapVenue
    ) external onlyOwnerOrLiquidator returns (int256 collateralGain) {
        uint256 collateralBalance = ERC20(collateralAsset).balanceOf(address(this));

        if (keccak256(abi.encodePacked(swapVenue)) == keccak256(abi.encodePacked("kittenswap"))) {
            swapOutKittenswap(
                debtToCover,
                SwapCallbackData({path: swapPath, collateralAsset: collateralAsset, debtAsset: debtAsset, user: user, debtToCover: debtToCover, amountToPay: 0})
            );
        } else if (keccak256(abi.encodePacked(swapVenue)) == keccak256(abi.encodePacked("hyperswap"))) {
            swapOutUniswap(
                debtToCover,
                SwapCallbackData({path: swapPath, collateralAsset: collateralAsset, debtAsset: debtAsset, user: user, debtToCover: debtToCover, amountToPay: 0})
            );
        } else {
            revert("Invalid swap venue");
        }

        collateralGain = int256(ERC20(collateralAsset).balanceOf(address(this))) - int256(collateralBalance);
    }

    /// @dev Performs a single exact output swap
    function swapOutKittenswap(uint256 amountOut, SwapCallbackData memory data) internal {
        (address tokenOut, address tokenIn, uint24 fee) = Path.decodeFirstPool(data.path);

        // path is reversed for exact output swaps
        bool zeroForOne = !(tokenIn < tokenOut);

        // storage is set for both directions; no address sorting necessary
        activeKittenPair = IKittenPair(IKittenPairFactory(kittenPairFactory).getPair(tokenIn, tokenOut, true));

        if (address(activeKittenPair) == address(0)) {
            revert("Invalid kitten pair");
        }

        data.amountToPay = KittenswapLib.getAmountIn(
            KittenswapLib.GetAmountInArgs({
                pair: address(activeKittenPair),
                factory: address(kittenPairFactory),
                amountOut: data.debtToCover,
                tokenOut: tokenIn // exact output swaps are reversed
            })
        );

        activeKittenPair.swap(
            zeroForOne ? 0 : amountOut,
            zeroForOne ? amountOut : 0,
            address(this),
            abi.encode(data)
        );

        activeKittenPair = IKittenPair(address(0));
    }

    /// @inheritdoc IKittenswapSwapCallback
    function hook(address sender, uint256 amount0Out, uint256 amount1Out, bytes calldata _data) external override {
        // validate msg.sender is kitten pair
        if (msg.sender != address(activeKittenPair)) {
            revert("msg.sender != activeKittenPair");
        }

        SwapCallbackData memory data = abi.decode(_data, (SwapCallbackData));

        (address tokenIn, address tokenOut, uint24 fee) = Path.decodeFirstPool(data.path);

        pool.liquidationCall(
            data.collateralAsset,
            data.debtAsset,
            data.user,
            data.debtToCover,
            false // receiveAToken is false so we can repay the swap
        );

        // either initiate the next swap or pay
        if (Path.hasMultiplePools(data.path)) {
            data.path = Path.skipToken(data.path);
            swapOutKittenswap(data.amountToPay, data);
        }

        tokenIn = tokenOut; // swap in/out because exact output swaps are reversed

        ERC20(tokenIn).transfer(msg.sender, data.amountToPay);
    }

    /// @dev Performs a single exact output swap
    function swapOutUniswap(uint256 amountOut, SwapCallbackData memory data) internal {
        (address tokenOut, address tokenIn, uint24 fee) = Path.decodeFirstPool(data.path);

        bool zeroForOne = tokenIn < tokenOut;

        IUniswapV3PoolActions uniswapPool = IUniswapV3PoolActions(
            PoolAddress.computeAddress(uniswapV3Factory, PoolAddress.getPoolKey(tokenIn, tokenOut, fee))
        );

        uniswapPool.token0();

        uniswapPool.swap(
            address(this),
            zeroForOne,
            -int256(amountOut),
            zeroForOne ? MIN_SQRT_RATIO + 1 : MAX_SQRT_RATIO - 1,
            abi.encode(data)
        );
    }

    /// @inheritdoc IUniswapV3SwapCallback
    function uniswapV3SwapCallback(int256 amount0Delta, int256 amount1Delta, bytes calldata _data) external override {
        SwapCallbackData memory data = abi.decode(_data, (SwapCallbackData));

        (address tokenIn, address tokenOut, uint24 fee) = Path.decodeFirstPool(data.path);
        verifyCallback(uniswapV3Factory, PoolAddress.getPoolKey(tokenIn, tokenOut, fee));

        pool.liquidationCall(
            data.collateralAsset,
            data.debtAsset,
            data.user,
            data.debtToCover,
            false // receiveAToken is false so we can repay the swap
        );

        uint256 amountToPay = amount0Delta > 0 ? uint256(amount0Delta) : uint256(amount1Delta);

        // either initiate the next swap or pay
        if (Path.hasMultiplePools(data.path)) {
            data.path = Path.skipToken(data.path);
            swapOutUniswap(amountToPay, data);
        }

        tokenIn = tokenOut; // swap in/out because exact output swaps are reversed

        ERC20(tokenIn).transfer(msg.sender, amountToPay);
    }

    /// @notice Approve max ERC-20 allowance to Aave pool to save gas and not have to approve every liquidation
    /// @param token address of ERC-20 to approve
    function approvePool(address token) external onlyOwnerOrLiquidator {
        ERC20(token).approve(address(pool), type(uint256).max);
    }

    /// @notice Withdraw from contract (only owner)
    /// @param token address of ERC20 to withdraw, zero address withdraws native asset (i.e.: ETH)
    /// @param amount amount to withdraw
    function recover(address token, uint256 amount) external onlyOwner {
        if (token == address(0)) {
            payable(msg.sender).transfer(amount);
            return;
        }
        ERC20(token).transfer(msg.sender, amount);
    }

    /// @notice Returns the address of a valid Uniswap V3 Pool
    /// @param factory The contract address of the Uniswap V3 factory
    /// @param poolKey The identifying key of the V3 pool
    function verifyCallback(address factory, PoolAddress.PoolKey memory poolKey) internal view {
        address p = PoolAddress.computeAddress(factory, poolKey);
        require(msg.sender == p, "invalid pool");
    }

    // function testGetAmountIn() external {
    //     uint256 amountOut = 1e18;
    //     activeKittenPair = IKittenPair(0x7F5D1F93232702a951759a47A27dE08D35e9bbe8);
    //     // 0x5555555555555555555555555555555555555555 for WHYPE
    //     //0x94e8396e0869c9F2200760aF0621aFd240E1CF38 for wstHYPE
    //     uint256 amountIn = KittenswapLib.getAmountIn(
    //         KittenswapLib.GetAmountInArgs({pair: address(activeKittenPair), factory: address(kittenPairFactory), amountOut: amountOut, tokenOut: address(0x94e8396e0869c9F2200760aF0621aFd240E1CF38)})
    //     );
    //     emit GetAmountInTest(amountIn);
    //     activeKittenPair = IKittenPair(address(0));
    // }
}
