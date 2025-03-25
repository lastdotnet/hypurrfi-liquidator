// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

interface IKittenPair {
    function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external;

    function metadata()
        external
        view
        returns (uint256 dec0, uint256 dec1, uint256 r0, uint256 r1, bool st, address t0, address t1);

    function token0() external view returns (address);

    function token1() external view returns (address);

    function reserve0() external view returns (uint256);

    function reserve1() external view returns (uint256);

    function getAmountOut(uint256 amountIn, address tokenIn) external view returns (uint256);
}
