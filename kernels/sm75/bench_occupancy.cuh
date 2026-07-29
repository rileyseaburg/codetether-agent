// Occupancy limits reported by the driver for sm_75.
//
// Shared memory per block bounds how many blocks an SM can host, which in
// turn bounds latency hiding. Attention tile geometry is chosen against these
// numbers rather than against Ampere's larger budget.
#pragma once

#include <cstdio>
#include <cuda_runtime.h>

#include "bench_bank_sweep.cuh"

inline void report_occupancy() {
    cudaDeviceProp prop{};
    cudaGetDeviceProperties(&prop, 0);

    int blocks_per_sm = 0;
    cudaOccupancyMaxActiveBlocksPerMultiprocessor(
        &blocks_per_sm, smem_stride_kernel<33>, 256, 0);

    // Largest square fp16 tile that fits one 48 KB block, holding Q, K, and V.
    const int bytes_per_element = 2;
    const int tiles_resident = 3;
    const int budget = prop.sharedMemPerBlock;
    int side = 0;
    while ((side + 16) * (side + 16) * bytes_per_element * tiles_resident <= budget) {
        side += 16;
    }

    printf("  \"smem_per_block_kb\": %d,\n", int(budget / 1024));
    printf("  \"smem_per_sm_kb\": %d,\n",
           int(prop.sharedMemPerMultiprocessor / 1024));
    printf("  \"blocks_per_sm\": %d,\n", blocks_per_sm);
    printf("  \"max_square_qkv_tile\": %d,\n", side);
    printf("  \"warps_per_sm\": %d\n",
           prop.maxThreadsPerMultiProcessor / prop.warpSize);
}