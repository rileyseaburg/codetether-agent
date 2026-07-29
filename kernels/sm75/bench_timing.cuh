// Timing helpers shared by the sm_75 microbenchmarks.
//
// Each measurement runs a warmup launch first so clocks settle, then reports
// the median of several timed launches to reduce boost-clock variance.
#pragma once

#include <algorithm>
#include <cuda_runtime.h>
#include <vector>

#include "bench_mma.cuh"

constexpr int kRepeats = 7;

template <typename Kernel>
float time_kernel(Kernel kernel, int blocks, int threads, int iters) {
    float *sink = nullptr;
    cudaMalloc(&sink, sizeof(float));
    kernel<<<blocks, threads>>>(iters, sink);
    cudaDeviceSynchronize();

    cudaEvent_t start, stop;
    cudaEventCreate(&start);
    cudaEventCreate(&stop);
    std::vector<float> samples;
    for (int r = 0; r < kRepeats; ++r) {
        cudaEventRecord(start);
        kernel<<<blocks, threads>>>(iters, sink);
        cudaEventRecord(stop);
        cudaEventSynchronize(stop);
        float ms = 0.0f;
        cudaEventElapsedTime(&ms, start, stop);
        samples.push_back(ms);
    }
    std::sort(samples.begin(), samples.end());
    cudaEventDestroy(start);
    cudaEventDestroy(stop);
    cudaFree(sink);
    return samples[samples.size() / 2];
}

inline float time_mma(int blocks, int threads, int iters) {
    return time_kernel(mma_kernel, blocks, threads, iters);
}
