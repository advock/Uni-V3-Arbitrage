// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IUniswapV3SwapCallback {
    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata data
    ) external;
}

interface IUniV3Pool {
    function swap(
        address recipient,
        bool zeroForOne,
        int256 amountSpecified,
        uint160 sqrtPriceLimitX96,
        bytes calldata data
    ) external returns (int256 amount0, int256 amount1);

    function token0() external view returns (address);

    function token1() external view returns (address);
}

interface IERC20 {
    function transfer(
        address recipient,
        uint256 amount
    ) external returns (bool);
    function balanceOf(address account) external view returns (uint256);
}

contract UniV3CycleExecutor is IUniswapV3SwapCallback {
    struct PoolSequence {
        address pool1;
        address pool2;
        address pool3;
    }

    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata _data
    ) external override {
        // Decode token0 and token1 addresses from the _data parameter
        (address token0, address token1) = abi.decode(
            _data,
            (address, address)
        );

        // If amount0Delta is positive, the pool expects token0 from this contract
        if (amount0Delta > 0) {
            require(
                IERC20(token0).transfer(msg.sender, uint256(amount0Delta)),
                "Transfer of token0 failed"
            );
        }

        // If amount1Delta is positive, the pool expects token1 from this contract
        if (amount1Delta > 0) {
            require(
                IERC20(token1).transfer(msg.sender, uint256(amount1Delta)),
                "Transfer of token1 failed"
            );
        }
    }

    // Function to perform the swaps across the cycle of 3 pools
    function executeCycleSwap(
        PoolSequence calldata pools,
        uint256 amountIn,
        bool firstSwapZeroForOne,
        address recipient
    ) external returns (int256 finalAmountOut) {
        // Perform the first swap on pool1
        uint256 amountOut1 = executeSwap(
            pools.pool1,
            firstSwapZeroForOne,
            amountIn,
            recipient
        );

        // Perform the second swap on pool2, using the output of the first swap
        uint256 amountOut2 = executeSwap(
            pools.pool2,
            !firstSwapZeroForOne,
            amountOut1,
            recipient
        );

        // Perform the third swap on pool3, using the output of the second swap
        uint256 amountOut3 = executeSwap(
            pools.pool3,
            firstSwapZeroForOne,
            amountOut2,
            recipient
        );

        return int256(amountOut3);
    }

    // Internal function to perform a real swap on a given pool
    function executeSwap(
        address pool,
        bool zeroForOne,
        uint256 amountIn,
        address recipient
    ) internal returns (uint256 amountOut) {
        // Set the price limit for the swap (using high values for simplicity)
        uint160 sqrtPriceLimitX96 = (
            zeroForOne
                ? 4295128749
                : 1461446703485210103287273052203988822378723970341
        );

        // Prepare the token addresses for the callback (used in `uniswapV3SwapCallback`)
        address token0 = zeroForOne
            ? IUniV3Pool(pool).token0()
            : IUniV3Pool(pool).token1();
        address token1 = zeroForOne
            ? IUniV3Pool(pool).token1()
            : IUniV3Pool(pool).token0();
        bytes memory data = abi.encode(token0, token1);

        // Perform the swap on the Uniswap V3 pool, specifying the recipient
        (int256 amount0, int256 amount1) = IUniV3Pool(pool).swap(
            recipient, // The address receiving the output of the swap
            zeroForOne, // Direction of the swap
            int256(amountIn), // The amount of input tokens
            sqrtPriceLimitX96,
            data // Pass token addresses for use in the callback
        );

        // Calculate the amount out from the swap
        amountOut = zeroForOne ? uint256(-amount1) : uint256(-amount0);
    }
}
