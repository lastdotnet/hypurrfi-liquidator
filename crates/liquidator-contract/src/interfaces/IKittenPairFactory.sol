// SPDX-License-Identifier: MIT
pragma solidity ^0.8.23;

interface IKittenPairFactory {
    function allPairsLength() external view returns (uint256);

    function isPair(address pair) external view returns (bool);

    function pairCodeHash() external pure returns (bytes32);

    function getPair(address tokenA, address token, bool stable) external view returns (address);

    function createPair(address tokenA, address tokenB, bool stable) external returns (address pair);

    function getFee(address _pair, bool _stable) external view returns (uint256);
}
