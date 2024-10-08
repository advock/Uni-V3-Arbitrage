// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IUniV3Pool {
    function swap(
        address recipient,
        bool zeroForOne,
        int256 amountSpecified,
        uint160 sqrtPriceLimitX96,
        bytes calldata data
    ) external returns (int256 amount0, int256 amount1);
}

contract UniV3CycleQuoter {
    struct PoolSequence {
        address pool1;
        address pool2;
        address pool3;
    }

    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata _data
    ) external {
        revert(string(abi.encode(amount0Delta, amount1Delta)));
    }

    function getCycleReturn(
        PoolSequence calldata pools,
        uint256 amountIn,
        bool firstSwapZeroForOne
    ) external returns (int256 finalAmountOut) {
        // Perform the first swap on pool1
        uint256 amountOut1 = getAmountOut(
            pools.pool1,
            firstSwapZeroForOne,
            amountIn
        );

        // Perform the second swap on pool2, using the output of the first swap
        uint256 amountOut2 = getAmountOut(
            pools.pool2,
            !firstSwapZeroForOne,
            amountOut1
        );

        // Perform the third swap on pool3, using the output of the second swap
        uint256 amountOut3 = getAmountOut(
            pools.pool3,
            firstSwapZeroForOne,
            amountOut2
        );

        return int256(amountOut3);
    }

    function getAmountOut(
        address pool,
        bool zeroForOne,
        uint256 amountIn
    ) internal returns (uint256 amountOut) {
        uint160 sqrtPriceLimitX96 = (
            zeroForOne
                ? 4295128749
                : 1461446703485210103287273052203988822378723970341
        );

        try
            IUniV3Pool(pool).swap(
                address(this),
                zeroForOne,
                int256(amountIn),
                sqrtPriceLimitX96,
                ""
            )
        returns (int256 amount0, int256 amount1) {
            amountOut = zeroForOne ? uint256(-amount1) : uint256(-amount0);
        } catch (bytes memory reason) {
            (int256 amount0Delta, int256 amount1Delta) = abi.decode(
                reason,
                (int256, int256)
            );
            amountOut = zeroForOne
                ? uint256(-amount1Delta)
                : uint256(-amount0Delta);
        }
    }
}
