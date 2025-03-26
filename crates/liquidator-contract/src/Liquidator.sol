// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Owned} from "solmate/auth/Owned.sol";
import {ERC20} from "solmate/tokens/ERC20.sol";
import {IPool} from "./interfaces/IPool.sol";
import {IUniswapV3SwapCallback} from "./interfaces/IUniswapV3SwapCallback.sol";
import {IUniswapV3PoolActions} from "./interfaces/IUniswapV3PoolActions.sol";
import {PoolAddress} from "./lib/PoolAddress.sol";
import {Path} from "./lib/Path.sol";
import {KittenPath} from "./lib/KittenPath.sol";
import {IKittenPair} from "./interfaces/IKittenPair.sol";
import {IKittenPairFactory} from "./interfaces/IKittenPairFactory.sol";
import {KittenswapLib} from "./lib/KittenswapLib.sol";
import {IKittenswapSwapCallback} from "./interfaces/IKittenswapSwapCallback.sol";
import {IERC20} from "./interfaces/IERC20.sol";
import {IUsdxlFlashMinter} from "./interfaces/IUsdxlFlashMinter.sol";
import {IERC3156FlashBorrower} from "./interfaces/IERC3156FlashBorrower.sol";

uint160 constant MIN_SQRT_RATIO = 4295128739;
/// @dev The maximum value that can be returned from #getSqrtRatioAtTick. Equivalent to getSqrtRatioAtTick(MAX_TICK)
uint160 constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970342;

contract Liquidator is Owned(msg.sender), IKittenswapSwapCallback, IUniswapV3SwapCallback, IERC3156FlashBorrower {
    event LiquidatorSet(address indexed liquidator, bool enabled);

    struct SwapCallbackData {
        bytes path;
        address collateralAsset;
        address debtAsset;
        address user;
        uint256 debtToCover;
        uint256 amountToPay;
        bool liquidateUser;
        bool swapOut;
    }

    // hypurrfi
    IPool public constant pool = IPool(0xceCcE0EB9DD2Ef7996e01e25DD70e461F918A14b);

    // usdxl
    IERC20 public constant USDXL = IERC20(0xca79db4B49f608eF54a5CB813FbEd3a6387bC645);
    IUsdxlFlashMinter public constant FLASH_MINTER = IUsdxlFlashMinter(0xD12f1c402197224339D5A324AC7ef4DF5d2142E9);

    // hyperswap
    address public constant hyperswapV3Factory = 0xB1c0fa0B789320044A6F623cFe5eBda9562602E3;

    // kittenswap
    IKittenPairFactory public constant kittenPairFactory =
        IKittenPairFactory(0xDa12F450580A4cc485C3b501BAB7b0B3cbc3B31B);
    IKittenPair private activeKittenPair;

    mapping(address => bool) public isLiquidator;

    constructor() {}

    modifier onlyOwnerOrLiquidator() {
        require(msg.sender == owner || isLiquidator[msg.sender], "Only owner or liquidator can call this function");
        _;
    }

    modifier noInt256Overflow(address collateralAsset, address debtAsset) {
        if (ERC20(collateralAsset).balanceOf(address(this)) > uint256(type(int256).max)) {
            revert("Collateral asset balance too large");
        }
        if (ERC20(debtAsset).balanceOf(address(this)) > uint256(type(int256).max)) {
            revert("Debt asset balance too large");
        }
        _;
    }

    /// @notice Enable or disable a liquidator
    /// @param _liquidator address of the liquidator
    /// @param _enabled true to enable, false to disable
    function setLiquidator(address _liquidator, bool _enabled) external onlyOwner {
        isLiquidator[_liquidator] = true;
        emit LiquidatorSet(_liquidator, _enabled);
    }

    /// @notice Performs a liquidation using a flash swap
    /// @param collateralAsset address of the collateral asset to be liquidated
    /// @param debtAsset address of the debt asset to be repaid
    /// @param user address of the user to be liquidated
    /// @param debtToCover amount of debt asset to repay in exchange for collateral
    /// @param swapPath encoded path of pools to swap collateral through, see: https://docs.uniswap.org/contracts/v3/guides/swaps/multihop-swaps
    /// @param liqPath either "kittenswap" or "hyperswap" or "usdxlFlashMinter"
    function liquidate(
        address collateralAsset,
        address debtAsset,
        address user,
        uint256 debtToCover,
        bytes calldata swapPath,
        string calldata liqPath
    )
        external
        onlyOwnerOrLiquidator
        noInt256Overflow(collateralAsset, debtAsset)
        returns (address finalToken, int256 finalGain)
    {
        if (keccak256(abi.encodePacked(liqPath)) == keccak256(abi.encodePacked("kittenswap"))) {
            // swap ends with collateral asset
            finalToken = collateralAsset;

            // get balance before liquidation
            finalGain = int256(ERC20(collateralAsset).balanceOf(address(this)));

            // execute liquidation and swap(s)
            _swapOutKittenswap(
                debtToCover,
                SwapCallbackData({
                    path: swapPath,
                    collateralAsset: collateralAsset,
                    debtAsset: debtAsset,
                    user: user,
                    debtToCover: debtToCover,
                    amountToPay: 0,
                    liquidateUser: true,
                    swapOut: true
                })
            );

            // calculate final gain
            finalGain = int256(ERC20(collateralAsset).balanceOf(address(this))) - finalGain;
        } else if (keccak256(abi.encodePacked(liqPath)) == keccak256(abi.encodePacked("hyperswap"))) {
            // swap ends with collateral asset
            finalToken = collateralAsset;

            // get balance before liquidation
            finalGain = int256(ERC20(collateralAsset).balanceOf(address(this)));

            // execute liquidation and swap(s)
            _swapOutUniswapV3(
                debtToCover,
                SwapCallbackData({
                    path: swapPath,
                    collateralAsset: collateralAsset,
                    debtAsset: debtAsset,
                    user: user,
                    debtToCover: debtToCover,
                    amountToPay: 0,
                    liquidateUser: true,
                    swapOut: true
                })
            );

            // calculate final gain
            finalGain = int256(ERC20(collateralAsset).balanceOf(address(this))) - finalGain;
        } else if (keccak256(abi.encodePacked(liqPath)) == keccak256(abi.encodePacked("usdxlFlashMinter"))) {
            // swap ends with debt asset (USDXL)
            finalToken = debtAsset;

            // get balance before flash loan
            finalGain = int256(ERC20(debtAsset).balanceOf(address(this)));

            // execute flash loan, liquidate, and swap to USDXL
            _flashLoanUsdxl(
                debtToCover,
                SwapCallbackData({
                    path: swapPath,
                    collateralAsset: collateralAsset,
                    debtAsset: debtAsset,
                    user: user,
                    debtToCover: debtToCover,
                    amountToPay: 0,
                    liquidateUser: false,
                    swapOut: false
                })
            );

            // calculate final gain
            finalGain = int256(ERC20(debtAsset).balanceOf(address(this))) - finalGain;
        } else {
            revert("Invalid liquidation path");
        }
    }

    struct SwapInKittenswapLocals {
        address tokenIn;
        address tokenOut;
        uint256 amountOut;
        uint24 fee;
        bool zeroForOne;
        bool stable;
    }

    function _swapInKittenswap(uint256 amountIn, SwapCallbackData memory data) internal {
        SwapInKittenswapLocals memory locals;

        (locals.tokenIn, locals.tokenOut, locals.stable) = KittenPath.decodeFirstPool(data.path);

        // TODO: configurable stable/volatile for pair; perhaps this is included in the swap path
        activeKittenPair = IKittenPair(kittenPairFactory.getPair(locals.tokenIn, locals.tokenOut, locals.stable));

        if (address(activeKittenPair) == address(0)) {
            revert("Invalid kitten pair");
        }

        // get amount out
        data.amountToPay = amountIn;

        locals.zeroForOne = (locals.tokenIn < locals.tokenOut);

        locals.amountOut = KittenswapLib.getAmountOut(
            KittenswapLib.GetAmountOutArgs({
                pair: address(activeKittenPair),
                factory: address(kittenPairFactory),
                amountIn: amountIn,
                tokenIn: locals.tokenIn
            })
        );

        activeKittenPair.swap(
            locals.zeroForOne ? 0 : locals.amountOut,
            locals.zeroForOne ? locals.amountOut : 0,
            address(this),
            abi.encode(data)
        );

        activeKittenPair = IKittenPair(address(0));
    }

    struct SwapOutKittenswapLocals {
        address tokenOut;
        address tokenIn;
        bool stable;
    }

    /// @dev Performs a single exact output swap
    function _swapOutKittenswap(uint256 amountOut, SwapCallbackData memory data) internal {
        SwapOutKittenswapLocals memory locals;

        (locals.tokenOut, locals.tokenIn, locals.stable) = KittenPath.decodeFirstPool(data.path);

        // path is reversed for exact output swaps
        bool zeroForOne = !(locals.tokenIn < locals.tokenOut);

        // storage is set for both directions; no address sorting necessary
        activeKittenPair = IKittenPair(kittenPairFactory.getPair(locals.tokenIn, locals.tokenOut, locals.stable));

        if (address(activeKittenPair) == address(0)) {
            revert("Invalid kitten pair");
        }

        data.amountToPay = KittenswapLib.getAmountIn(
            KittenswapLib.GetAmountInArgs({
                pair: address(activeKittenPair),
                factory: address(kittenPairFactory),
                amountOut: data.debtToCover,
                tokenOut: locals.tokenIn // exact output swaps are reversed
            })
        );

        activeKittenPair.swap(zeroForOne ? 0 : amountOut, zeroForOne ? amountOut : 0, address(this), abi.encode(data));

        activeKittenPair = IKittenPair(address(0));
    }

    struct KittenswapHookLocals {
        address tokenIn;
        address tokenOut;
        bool stable;
        SwapCallbackData data;
    }

    /// @inheritdoc IKittenswapSwapCallback
    function hook(address, uint256, uint256, bytes calldata data) external override {
        // validate msg.sender is kitten pair
        if (msg.sender != address(activeKittenPair)) {
            revert("msg.sender != activeKittenPair");
        }

        KittenswapHookLocals memory locals;

        locals.data = abi.decode(data, (SwapCallbackData));

        (locals.tokenIn, locals.tokenOut, locals.stable) = KittenPath.decodeFirstPool(locals.data.path);

        if (locals.data.liquidateUser) {
            pool.liquidationCall(
                locals.data.collateralAsset,
                locals.data.debtAsset,
                locals.data.user,
                locals.data.debtToCover,
                false // receiveAToken is false so we can repay the swap
            );
        }

        // either initiate the next swap or pay
        if (KittenPath.hasMultiplePools(locals.data.path)) {
            locals.data.path = KittenPath.skipToken(locals.data.path);
            if (locals.data.swapOut) {
                _swapOutKittenswap(locals.data.amountToPay, locals.data);
            } else {
                _swapInKittenswap(locals.data.amountToPay, locals.data);
            }
        }

        if (locals.data.swapOut) {
            locals.tokenIn = locals.tokenOut; // swap in/out because exact output swaps are reversed
        }

        ERC20(locals.tokenIn).transfer(msg.sender, locals.data.amountToPay);
    }

    /// @dev Performs a single exact output swap
    function _swapOutUniswapV3(uint256 amountOut, SwapCallbackData memory data) internal {
        (address tokenOut, address tokenIn, uint24 fee) = Path.decodeFirstPool(data.path);

        bool zeroForOne = tokenIn < tokenOut;

        IUniswapV3PoolActions uniswapPool = IUniswapV3PoolActions(
            PoolAddress.computeAddress(hyperswapV3Factory, PoolAddress.getPoolKey(tokenIn, tokenOut, fee))
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
        verifyCallback(hyperswapV3Factory, PoolAddress.getPoolKey(tokenIn, tokenOut, fee));

        if (data.liquidateUser) {
            pool.liquidationCall(
                data.collateralAsset,
                data.debtAsset,
                data.user,
                data.debtToCover,
                false // receiveAToken is false so we can repay the swap
            );
        }

        uint256 amountToPay = amount0Delta > 0 ? uint256(amount0Delta) : uint256(amount1Delta);

        // either initiate the next swap or pay
        if (Path.hasMultiplePools(data.path)) {
            data.path = Path.skipToken(data.path);
            _swapOutUniswapV3(amountToPay, data);
        }

        tokenIn = tokenOut; // swap in/out because exact output swaps are reversed

        ERC20(tokenIn).transfer(msg.sender, amountToPay);
    }

    /**
     * @notice Initiates a flash loan
     * @param debtToCover The amount of USDXL debt to cover
     * @param data Additional data passed to the flash loan
     */
    function _flashLoanUsdxl(uint256 debtToCover, SwapCallbackData memory data) internal {
        FLASH_MINTER.flashLoan(IERC3156FlashBorrower(address(this)), address(USDXL), debtToCover, abi.encode(data));
    }

    struct ExecuteOperationLocals {
        uint256 amountToRepay;
        uint256 collateralGained;
        SwapCallbackData data;
    }

    bytes32 public constant CALLBACK_SUCCESS = keccak256('ERC3156FlashBorrower.onFlashLoan');

    /**
     * @notice Callback function called by the flash minter
     * @param initiator The address that initiated the flash loan
     * @param amount The amount of USDXL borrowed
     * @param fee The fee to be paid for the flash loan
     * @param data Additional data passed to the flash loan
     * @return success The keccak256 hash of "IERC3156FlashBorrower.onFlashLoan"
     */
    function onFlashLoan(address initiator, address token, uint256 amount, uint256 fee, bytes calldata data) external returns (bytes32 success) {
        // Ensure caller is the flash minter
        require(initiator == address(this), "Initiator of onFlashLoan() must be liquidator contract");
        require(msg.sender == address(FLASH_MINTER), "Msg.sender of onFlashLoan() must be usdxlFlashMinter");
        require(token == address(USDXL), "Flash loaned token must be USDXL");

        // Verify we received the flash loaned amount
        require(USDXL.balanceOf(address(this)) >= amount, "Invalid balance for flash loan");

        ExecuteOperationLocals memory locals;

        locals.data = abi.decode(data, (SwapCallbackData));

        require(locals.data.debtAsset == address(USDXL), "Debt asset must be USDXL");

        // Calculate total amount to repay (loan + fee)
        locals.amountToRepay = amount + fee;

        locals.collateralGained = ERC20(locals.data.collateralAsset).balanceOf(address(this));

        // liquidate user to receive collateral
        pool.liquidationCall(
            locals.data.collateralAsset,
            locals.data.debtAsset,
            locals.data.user,
            locals.data.debtToCover,
            false // receiveAToken is false so we can repay the swap
        );

        locals.collateralGained = ERC20(locals.data.collateralAsset).balanceOf(address(this)) - locals.collateralGained;

        // swap all collateral to USDXL
        _swapInKittenswap(locals.collateralGained, locals.data);

        // Approve flash minter to pull repayment
        USDXL.approve(address(FLASH_MINTER), locals.amountToRepay);

        return CALLBACK_SUCCESS;
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
