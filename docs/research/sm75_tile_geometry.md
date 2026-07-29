# sm_75 tile geometry, measured

Evidence level: **live hardware**. RTX 2080 SUPER, sm_75, `nvcc` 12.0,
`-O3 -arch=sm_75`, median of 7 timed launches after a warmup launch.

## Bank conflicts

With the column index shared across a warp, the bank is
`(lane * stride) % 32`. Sweeping the row stride isolates the effect:

| Row stride | Time (ms) | Penalty vs 33 | Predicted ways |
|---:|---:|---:|---:|
| 32 | 203.889 | **31.54x** | 32 |
| 33 | 6.465 | 1.00 | 1 |
| 34 | 12.999 | **2.01x** | 2 |

Measured penalties of 31.54x and 2.01x match the predicted 32-way and 2-way
serialization. `gcd(stride, 32)` gives the conflict degree, so odd strides are
conflict-free and even strides serialize by their common factor.

**Consequence:** attention tiles must use an odd row stride. A natural
`head_dim = 64` layout is the worst possible case at 32-way serialization.

### Two measurement errors worth recording

A first attempt reported a penalty of 1.00 because the access offsets were
loop-invariant and the compiler hoisted the shared loads out of the timing
loop. A second attempt made the index data-dependent but indexed
`tile[lane * 32 + col]` with a varying `col`, which already spreads lanes
across banks and therefore never conflicted. Only holding the column fixed
while varying the row exposes the conflict.

A penalty of 1.00 on 32 banks is physically implausible, and that was the
signal that the benchmark rather than the hardware was wrong.

## Occupancy and capacity

```json
{
  "smem_per_block_kb": 48,
  "smem_per_sm_kb": 64,
  "blocks_per_sm": 4,
  "max_square_qkv_tile": 80,
  "warps_per_sm": 32
}
```

Shared memory is the binding constraint. At 48 KB per block and 64 KB per SM,
only one block per SM can hold a full 48 KB allocation, so a kernel that
claims the maximum forfeits latency hiding entirely. Four 256-thread blocks
fit only when each stays near 16 KB.

`max_square_qkv_tile: 80` is the largest square fp16 tile holding Q, K, and V
in one 48 KB block. Rounded to a multiple of 16 for `mma.sync.m16n8k8`, that
leaves 64 or 80 as the practical tile side.

## Tensor core versus exponential

```json
{
  "native_exp_gops": 975.3,
  "poly_exp_gops": 444.4,
  "mma_tflops": 45.59,
  "poly_over_native": 0.456
}
```

Reproduced across runs: `poly_over_native` was 0.456 in both, and
`mma_tflops` varied 39.97 to 45.59 with boost clock.

## Resulting design constraints

FA-4's forward-pass strategy assumes the exponential is scarce and matmul is
abundant. On sm_75 that ordering reverses, so the design targets differ:

1. Keep the exponential on the SFU. It costs about 47 tensor-core FLOPs per
   call here, against roughly two orders of magnitude more on B200.
2. Use an odd shared-memory row stride. Padding 64 to 65 or 72 to 73 avoids a
   measured 31.54x penalty.
3. Budget shared memory near 16 KB per block, not 48 KB, so four blocks per SM
   remain resident.
4. Tile side 64 or 80, aligned to the `m16n8k8` tensor-core shape.
5. There is no `cp.async`, so global-to-shared staging must be register-based.

## Caveats

`mma_tflops` measures back-to-back `mma.sync.m16n8k8` with register-resident
operands and no memory traffic, so it is an issue-rate ceiling rather than
achievable attention throughput. The bank sweep measures dependent scalar
loads and reports relative penalties, not absolute bandwidth.

Reproduce with `kernels/sm75/run_bench.sh`.
