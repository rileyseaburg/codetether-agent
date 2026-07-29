# SOTA algorithms under sm_75 constraints

The research contribution is adaptation. Published kernels target Ampere,
Hopper, and Blackwell. Our device is Turing sm_75, which removes primitives
those kernels assume. Each adaptation below is a real design problem with a
measurable outcome, not a port.

## Measured device limits

```json
{
  "name": "NVIDIA GeForce RTX 2080 SUPER",
  "capability": "7.5",
  "sm_count": 48,
  "shared_per_block_kb": 48,
  "shared_per_sm_kb": 64,
  "regs_per_sm": 65536,
  "max_threads_per_sm": 1024,
  "total_memory_gb": 8.16
}
```

Read from the device, not assumed.

## What sm_75 does not have

| Primitive | First available | Consequence |
|---|---|---|
| `cp.async` | sm_80 | no asynchronous global-to-shared copy; software pipelining must use registers |
| bfloat16 | sm_80 | fp16 with fp32 accumulation, and overflow must be managed explicitly |
| FP8 | sm_89 | low-precision numerics research is unavailable |
| TMA | sm_90 | descriptor-based bulk copy unavailable; addressing is manual |
| Warp-group MMA | sm_90 | tensor-core work is per-warp `mma.sync` |
| Distributed shared memory | sm_90 | no cross-block cooperation inside a cluster |
| 164 KB shared/SM | sm_80 | 64 KB/SM, and only 48 KB addressable per block |

`mma.sync.m16n8k8` for fp16 is available and is the largest tensor-core tile
we can issue.

## Adaptation targets

### FlashAttention without async copy

The published algorithm overlaps a `cp.async` global load with tensor-core
math on the previous tile. Without `cp.async` the load is synchronous, so the
pipeline has to be rebuilt around register-staged prefetch, and the tile size
is bounded by 48 KB per block rather than 164 KB.

Open question: at what head dimension does the reduced tile force enough extra
passes over K and V that a fused kernel stops beating a two-pass baseline.

### Chunked linear attention within 48 KB

Gated linear attention and DeltaNet use chunked scans sized for Ampere shared
memory. With 48 KB per block the chunk length, state tile, and accumulator must
be co-sized under one budget.

Open question: whether recomputing the state at chunk boundaries costs less
than spilling it to global memory, which is a different tradeoff at 48 KB than
at 164 KB.

### Blelloch scan for the affine monoid

The recurrence `S_t = a_t * S_{t-1} + v_t k_t^T` composes as
`(a_i, B_i) @ (a_j, B_j) = (a_i*a_j, a_i*B_j + B_i)`, which is associative and
not commutative. The scan is therefore valid but order-sensitive, so warp
shuffle reductions must preserve sequence order.

Turing has independent thread scheduling and full-warp `__shfl_sync`, so a
warp-level scan is available without cooperative groups.

### Muon and Newton-Schulz orthogonalization

Muon orthogonalizes momentum through a fixed number of Newton-Schulz
iterations, each a pair of small matrix products. This is compute-bound on
small matrices and fits sm_75 well, since it needs no async copy and no large
shared tiles.

Open question: the smallest iteration count and lowest precision that keeps
orthogonality error acceptable when accumulating in fp32 without bf16.

### Paged and quantized inference in 8 GB

An 8.16 GB device holding a 9B model in 4-bit leaves roughly 0.6 GB after
weights. KV-cache paging, block size, and eviction all become binding rather
than incidental.

Measured reference point: `Qwen3.5-9B` in 4-bit NF4 occupied 7520 MiB of 8192
MiB with an 8,192-token context.

## Method

Every adaptation follows the same sequence, so that a performance claim always
has a correctness oracle behind it:

1. Scalar reference in TetherScript, executed on the tree-walking interpreter.
2. Finite-difference gradient checks against that reference.
3. CUDA kernel exposed as a capability-gated native.
4. Differential test: kernel against interpreter on identical inputs.
5. Roofline measurement, reporting achieved bandwidth and FLOPs against the
   sm_75 ceiling.

Step 5 is what turns an adaptation into a result. Steps 1 and 2 require no
GPU.

## Baseline discipline

A claim of the form "our variant is faster" requires the unmodified algorithm
measured on the same device, same harness, same inputs. Absent that, the
comparison is unfalsifiable.

Loss curves are not evidence of capability. During earlier fine-tuning work,
validation loss fell from 1.5960 to 1.3955 across six checkpoints while
measured tool-call rate dropped from 0.875 to 0.250 on this same GPU.
Behavioural measurement is mandatory.
