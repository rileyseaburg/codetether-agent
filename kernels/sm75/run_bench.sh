#!/usr/bin/env bash
# Build and run the sm_75 microbenchmarks on the GPU host.
#
# Establishes whether FlashAttention-4's polynomial-exp substitution is
# profitable on Turing. FA-4 adopts it because Blackwell's SFU saturates
# before its tensor cores do; Turing's ratio is different and is measured
# here rather than assumed.
set -euo pipefail

root=/home/ubuntu/ct-eval/kernels
cd "$root"

nvcc -O3 -arch=sm_75 -std=c++17 \
    -o bench_exp_vs_mma bench_exp_vs_mma.cu

./bench_exp_vs_mma
