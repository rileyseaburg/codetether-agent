// Shared-memory bank behaviour and occupancy on sm_75.
//
// FA-4 treats shared-memory traffic as the Blackwell backward-pass bottleneck
// with 164 KB per SM. Turing has 64 KB per SM and 48 KB per block, so tile
// geometry is bounded earlier and padding choices matter more.
//
// The stride sweep isolates bank conflicts: with the column shared across a
// warp, bank index is (lane * stride) % 32, so stride 32 serializes and
// stride 33 does not.

#include <cstdio>

#include "bench_bank_sweep.cuh"
#include "bench_occupancy.cuh"
#include "bench_timing.cuh"

int main() {
    const int blocks = 48 * 4;
    const int threads = 256;
    const int iters = 200000;

    float s32 = time_kernel(smem_stride_kernel<32>, blocks, threads, iters);
    float s33 = time_kernel(smem_stride_kernel<33>, blocks, threads, iters);
    float s34 = time_kernel(smem_stride_kernel<34>, blocks, threads, iters);

    printf("{\n");
    printf("  \"stride32_ms\": %.3f,\n", s32);
    printf("  \"stride33_ms\": %.3f,\n", s33);
    printf("  \"stride34_ms\": %.3f,\n", s34);
    printf("  \"conflict_penalty_32_over_33\": %.2f,\n", s32 / s33);
    printf("  \"penalty_34_over_33\": %.2f,\n", s34 / s33);
    report_occupancy();
    printf("}\n");
    return 0;
}
