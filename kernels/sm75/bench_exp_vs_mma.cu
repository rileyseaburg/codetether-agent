// Measure the sm_75 ratio that FlashAttention-4 depends on.
//
// FA-4 (arXiv 2603.05451) emulates exp with an FMA polynomial because on
// Blackwell the SFU is the forward-pass bottleneck: BF16 tensor core
// throughput rose from 1 to 2.25 PFLOPs between H100 and B200 while SFU
// count and shared-memory bandwidth stayed flat.
//
// Turing scales the other way. Tensor cores are comparatively weak, so the
// same substitution may cost more than it saves. This measures the three
// throughputs that decide it, rather than assuming Blackwell's ratio holds.

#include <cstdio>
#include <cuda_fp16.h>

#include "bench_exp.cuh"
#include "bench_mma.cuh"
#include "bench_timing.cuh"

int main() {
    const int blocks = 48 * 4;
    const int threads = 256;
    const int iters = 4096;

    float native_ms = time_kernel(native_exp_kernel, blocks, threads, iters);
    float poly_ms = time_kernel(poly_exp_kernel, blocks, threads, iters);
    float mma_ms = time_mma(blocks, threads, iters);

    const double ops = double(blocks) * threads * iters * 32.0;
    printf("{\n");
    printf("  \"native_exp_gops\": %.1f,\n", ops / (native_ms * 1e6));
    printf("  \"poly_exp_gops\": %.1f,\n", ops / (poly_ms * 1e6));
    printf("  \"mma_tflops\": %.2f,\n", mma_tflops(blocks, threads, iters, mma_ms));
    printf("  \"poly_over_native\": %.3f\n", native_ms / poly_ms);
    printf("}\n");
    return 0;
}
