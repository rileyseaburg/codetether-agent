// Exponential throughput: hardware SFU versus FMA polynomial.
//
// FA-4 replaces `exp` with a degree-based polynomial on FMA units because
// Blackwell's SFU is saturated before its tensor cores are. Whether that
// trade is profitable on sm_75 is an empirical question, so both paths are
// written to be measured against each other.
#pragma once

#include <cuda_runtime.h>

// Hardware path: `__expf` lowers to MUL plus ex2.approx on the SFU.
__global__ void native_exp_kernel(int iters, float *sink) {
    float x = 0.01f * float(threadIdx.x);
    float acc = 0.0f;
#pragma unroll 8
    for (int i = 0; i < iters; ++i) {
        for (int lane = 0; lane < 32; ++lane) {
            acc += __expf(x + float(lane) * 1e-4f);
        }
        x = acc * 1e-8f;
    }
    if (threadIdx.x == 1024) {
        *sink = acc;
    }
}

// Polynomial path: exp(x) = 2^(x * log2e) split into integer and fractional
// parts, with the fraction approximated by a degree-5 minimax polynomial
// evaluated in Horner form on FMA units, and the integer part folded into
// the exponent field by bit manipulation.
__device__ __forceinline__ float poly_exp(float x) {
    const float log2e = 1.4426950408889634f;
    float t = x * log2e;
    float n = __builtin_floorf(t);
    float f = t - n;
    float p = 0.0001530901f;
    p = __fmaf_rn(p, f, 0.0013422634f);
    p = __fmaf_rn(p, f, 0.0096181295f);
    p = __fmaf_rn(p, f, 0.0555041087f);
    p = __fmaf_rn(p, f, 0.2402265069f);
    p = __fmaf_rn(p, f, 0.6931472180f);
    p = __fmaf_rn(p, f, 1.0000000000f);
    int exponent = int(n) << 23;
    float scale = __int_as_float((127 << 23) + exponent);
    return p * scale;
}

__global__ void poly_exp_kernel(int iters, float *sink) {
    float x = 0.01f * float(threadIdx.x);
    float acc = 0.0f;
#pragma unroll 8
    for (int i = 0; i < iters; ++i) {
        for (int lane = 0; lane < 32; ++lane) {
            acc += poly_exp(x + float(lane) * 1e-4f);
        }
        x = acc * 1e-8f;
    }
    if (threadIdx.x == 1024) {
        *sink = acc;
    }
}
