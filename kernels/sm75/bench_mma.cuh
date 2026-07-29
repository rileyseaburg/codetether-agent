// Tensor-core throughput on sm_75 via mma.sync.m16n8k8.
//
// Turing's largest fp16 tensor-core tile. Blackwell issues asynchronous
// tcgen05.mma against 256 KB of tensor memory per SM; sm_75 has neither, so
// this measures the ceiling the polynomial-exp trade must be judged against.
#pragma once

#include <cuda_fp16.h>
#include <cuda_runtime.h>

__global__ void mma_kernel(int iters, float *sink) {
    // m16n8k8: each warp holds 4 A halves, 2 B halves, 4 float accumulators.
    unsigned a0 = 0x3C003C00u;
    unsigned a1 = 0x3C003C00u;
    unsigned b0 = 0x3C003C00u;
    float c0 = 0.0f, c1 = 0.0f, c2 = 0.0f, c3 = 0.0f;

#pragma unroll 4
    for (int i = 0; i < iters; ++i) {
        asm volatile(
            "mma.sync.aligned.m16n8k8.row.col.f32.f16.f16.f32 "
            "{%0,%1,%2,%3}, {%4,%5}, {%6}, {%0,%1,%2,%3};\n"
            : "+f"(c0), "+f"(c1), "+f"(c2), "+f"(c3)
            : "r"(a0), "r"(a1), "r"(b0));
    }
    if (threadIdx.x == 1024) {
        *sink = c0 + c1 + c2 + c3;
    }
}

// m16n8k8 performs 16*8*8 multiply-accumulates, so 2*1024 flops per warp.
inline double mma_tflops(int blocks, int threads, int iters, float ms) {
    const double warps = double(blocks) * (threads / 32);
    const double flops = warps * iters * 2.0 * 16.0 * 8.0 * 8.0;
    return flops / (ms * 1e9);
}
