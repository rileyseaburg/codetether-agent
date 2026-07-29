// Bank-conflict sweep across shared-memory row strides on sm_75.
//
// Two earlier attempts reported a penalty near 1.0. The first was hoisted by
// the compiler; the second used `tile[lane * 32 + col]`, where the varying
// column already spreads lanes across banks, so it never conflicted.
//
// Turing has 32 banks of 4 bytes. A warp conflicts when several lanes address
// the same bank, that is when lanes differ by a multiple of 32 floats. The
// conflicting pattern therefore has to hold the column fixed and vary only
// the row: `tile[lane * stride + fixed]` with stride divisible by 32.
//
// Sweeping the stride shows the effect directly instead of asserting it.
#pragma once

#include <cuda_runtime.h>

constexpr int kSweepRows = 32;

template <int Stride>
__global__ void smem_stride_kernel(int iters, float *sink) {
    __shared__ float tile[kSweepRows * Stride];
    for (int i = threadIdx.x; i < kSweepRows * Stride; i += blockDim.x) {
        tile[i] = 1.0f + 1e-6f * float(i);
    }
    __syncthreads();

    const int lane = threadIdx.x & 31;
    float acc = 0.0f;
    int col = 0;
    for (int i = 0; i < iters; ++i) {
        // Row varies per lane, column is shared, so bank equals
        // (lane * Stride + col) % 32. Stride 32 collapses every lane onto one
        // bank; stride 33 spreads them across all 32.
        acc += tile[lane * Stride + col];
        col = (col + 1 + (int(acc) & 0)) & 15;
    }
    if (threadIdx.x == 1024) {
        *sink = acc;
    }
}
