// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IKittenPair} from "../interfaces/IKittenPair.sol";
import {IKittenPairFactory} from "../interfaces/IKittenPairFactory.sol";
import {IERC20} from "../interfaces/IERC20.sol";

library KittenswapLib {
    event Test(uint256 amountIn);

    struct Metadata {
        uint decimals0;
        uint decimals1;
        uint reserve0;
        uint reserve1;
        bool stable;
        address token0;
        address token1;
    }

    struct GetAmountInArgs {
        address pair;
        address factory;
        uint amountOut;
        address tokenOut;
    }

    struct GetXArgs {
        uint256 y0;
        uint256 xy;
        uint256 x;
        bool stable;
        uint256 precision0;
        uint256 precision1;
    }

    struct GetKArgs {
        uint256 x;
        uint256 y;
        bool stable;
        uint256 precision0;
        uint256 precision1;
    }

    function getAmountIn(
        GetAmountInArgs memory args
    ) internal view returns (uint256) {
        Metadata memory metadata;

        (
            metadata.decimals0,
            metadata.decimals1,
            metadata.reserve0,
            metadata.reserve1,
            metadata.stable,
            metadata.token0,
            metadata.token1
        ) = IKittenPair(args.pair).metadata();

        // Reverse the _getAmountOut calculation to get the pre-fee amountIn
        uint256 amountIn = _getAmountIn(args, metadata);

        // console.log("before fee:", amountIn);

        // Now account for the fee and adjust amountIn accordingly
        uint256 fee = IKittenPairFactory(args.factory).getFee(
            args.pair,
            metadata.stable
        );
        amountIn = (amountIn * 10000) / (10000 - fee); // Add fee back to get the total input

        // console.log("after fee:", amountIn);

        return amountIn;
    }
    
    function _getAmountIn(
        GetAmountInArgs memory args,
        Metadata memory metadata
    ) internal pure returns (uint256) {
        if (metadata.stable) {
            uint256 xy = _k(
                GetKArgs({
                    x: metadata.reserve0,
                    y: metadata.reserve1,
                    stable: metadata.stable,
                    precision0: metadata.decimals0,
                    precision1: metadata.decimals1
                })
            );
            metadata.reserve0 = (metadata.reserve0 * 1e18) / metadata.decimals0;
            metadata.reserve1 = (metadata.reserve1 * 1e18) / metadata.decimals1;
            (uint256 reserveA, uint256 reserveB) = args.tokenOut == metadata.token1
                ? (metadata.reserve0, metadata.reserve1)
                : (metadata.reserve1, metadata.reserve0);
            args.amountOut = args.tokenOut == metadata.token0
                ? (args.amountOut * 1e18) / metadata.decimals0
                : (args.amountOut * 1e18) / metadata.decimals1;

            // Solve for amountIn:
            uint256 y = reserveB - args.amountOut;
            uint256 amountIn = _get_x(
                GetXArgs({
                    y0: y,
                    xy: xy,
                    x: reserveA,
                    stable: metadata.stable,
                    precision0: metadata.decimals0,
                    precision1: metadata.decimals1
                })
            ) - reserveA;

            return
                (amountIn *
                    (
                        args.tokenOut == metadata.token0
                            ? metadata.decimals1
                            : metadata.decimals0
                    )) / 1e18;
        } else {
            (uint256 reserveA, uint256 reserveB) = args.tokenOut == metadata.token1
                ? (metadata.reserve0, metadata.reserve1)
                : (metadata.reserve1, metadata.reserve0);

            // console.log("reserveA", reserveA);
            // console.log("reserveB", reserveB);
            // console.log("amountOut", amountOut);

            // Solve for amountIn based on constant product formula:
            return (args.amountOut * reserveA) / (reserveB - args.amountOut);
        }
    }
    
    
    function _get_x(
        GetXArgs memory xArgs
    ) internal pure returns (uint256) {
        for (uint256 i = 0; i < 255; i++) {
            uint256 k = _f(xArgs.x, xArgs.y0);
            if (k < xArgs.xy) {
                // Similar logic as in _get_y, we adjust x upward if k < xy
                uint256 dx = ((xArgs.xy - k) * 1e18) / _d(xArgs.x, xArgs.y0);
                if (dx == 0) {
                    if (k == xArgs.xy) {
                        return xArgs.x; // Found the correct answer
                    }
                    if (_k(GetKArgs({
                        x: xArgs.x + 1,
                        y: xArgs.y0,
                        stable: xArgs.stable,
                        precision0: xArgs.precision0,
                        precision1: xArgs.precision1
                    })) > xArgs.xy) {
                        return xArgs.x + 1; // Return x + 1 if it's the closest answer
                    }
                    dx = 1;
                }
                xArgs.x = xArgs.x + dx;
            } else {
                // Adjust x downward if k > xy
                uint256 dx = ((k - xArgs.xy) * 1e18) / _d(xArgs.x, xArgs.y0);
                if (dx == 0) {
                    if (k == xArgs.xy || _f(xArgs.x - 1, xArgs.y0) < xArgs.xy) {
                        return xArgs.x; // Return x if it's the closest answer
                    }
                    dx = 1;
                }
                xArgs.x = xArgs.x - dx;
            }
        }
        revert("!x");
    }
    
    function _k(
        GetKArgs memory kArgs
    ) internal pure returns (uint256) {
        if (kArgs.stable) {
            uint256 _x = (kArgs.x * 1e18) / kArgs.precision0;
            uint256 _y = (kArgs.y * 1e18) / kArgs.precision1;
            uint256 _a = (_x * _y) / 1e18;
            uint256 _b = ((_x * _x) / 1e18 + (_y * _y) / 1e18);
            return (_a * _b) / 1e18; // x3y+y3x >= k
        } else {
            return kArgs.x * kArgs.y; // xy >= k
        }
    }
    
    function _f(uint256 x0, uint256 y) internal pure returns (uint256) {
        uint256 _a = (x0 * y) / 1e18;
        uint256 _b = ((x0 * x0) / 1e18 + (y * y) / 1e18);
        return (_a * _b) / 1e18;
    }
    
    function _d(uint256 x0, uint256 y) internal pure returns (uint256) {
        return
            (3 * x0 * ((y * y) / 1e18)) /
            1e18 +
            ((((x0 * x0) / 1e18) * x0) / 1e18);
    }
}
