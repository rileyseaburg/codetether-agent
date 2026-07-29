# FlashAttention-4 on sm_75: the exponential trade inverts

Evidence level: **live hardware**. Measured on an RTX 2080 SUPER (sm_75, 48
SMs) with `nvcc` 12.0, `-O3 -arch=sm_75`, median of 7 timed launches after a
warmup launch.

## What FA-4 assumes

FlashAttention-4 (arXiv 2603.05451) is built around *asymmetric hardware
scaling*. From H100 to B200, BF16 tensor-core throughput rises from 1 to 2.25
PFLOPs while SFU count and shared-memory bandwidth stay unchanged. The
forward-pass bottleneck therefore moves off the tensor cores and onto the
special function units computing `exp`.

FA-4's response is to stop using the SFU: it emulates the exponential with a
polynomial evaluated on FMA units, trading abundant matmul-adjacent FMA
throughput for scarce SFU throughput.

## What sm_75 actually measures

```json
{
  "native_exp_gops": 977.8,
  "poly_exp_gops": 445.8,
  "mma_tflops": 39.97,
  "poly_over_native": 0.456
}
```

The polynomial is **2.2x slower** than the hardware exponential on Turing.
FA-4's substitution is a pessimization here.

## Why the conclusion reverses

The ratio that matters is tensor-core throughput against exponential
throughput:

| Device | Tensor core | Exp path | Ratio |
|---|---:|---:|---|
| B200 (FA-4 target) | 2250 TFLOP/s | flat SFU | matmul greatly outpaces exp |
| RTX 2080 SUPER | **39.97 TFLOP/s** | **977.8 Gexp/s** | matmul is the scarce resource |

Turing's tensor cores are roughly 56x weaker than B200's, while its SFU is
not correspondingly weaker. The bottleneck is therefore the MMA pipeline, not
the exponential. Spending FMA slots to relieve the SFU relieves a resource
that is not constrained, and steals issue slots from the one that is.

At 39.97 TFLOP/s and 977.8 Gexp/s, one exponential costs about the same as 41
tensor-core FLOPs. On B200 the same comparison is roughly two orders of
magnitude in the other direction.

## Which FA-4 techniques survive

| FA-4 technique | Depends on | sm_75 |
|---|---|---|
| Polynomial exp emulation | SFU-bound forward pass | **rejected: 2.2x slower** |
| Fully asynchronous MMA pipelining | `tcgen05.mma`, sm_90 | unavailable |
| Tensor memory for backward intermediates | 256 KB TMEM per SM, sm_90 | unavailable |
| 2-CTA MMA mode | Blackwell clusters | unavailable |
| Conditional online softmax rescaling | arithmetic only | **portable and worth testing** |
| Tile scheduler for causal load imbalance | scheduling only | **portable and worth testing** |
| CuTe-DSL implementation | compile-time concern | not applicable |

Two of seven techniques transfer. Both are algorithmic rather than
hardware-dependent, which is the useful dividing line.

## Consequence for a Turing attention kernel

Because the tensor core is the scarce resource, the design targets change:

- Keep `exp` on the SFU. It is effectively free relative to MMA.
- Optimize for MMA issue efficiency and occupancy, not for reduced
  non-matmul work.
- Respect 48 KB shared memory per block against Ampere's 164 KB, which bounds
  tile size before any pipelining choice is made.
- There is no `cp.async`, so global-to-shared staging must be register-based
  and cannot be overlapped the way FA-3 and FA-4 assume.

## Measurement caveats

`mma_tflops` measures back-to-back `mma.sync.m16n8k8` with operands resident
in registers and no memory traffic, so it is an issue-rate ceiling rather
than an achievable attention throughput. The exponential benchmarks likewise
exclude memory movement. The comparison is valid for deciding the SFU-versus-
FMA trade, which is what it was written for, and is not a substitute for
end-to-end kernel measurement.

Reproduce with `kernels/sm75/run_bench.sh`.
