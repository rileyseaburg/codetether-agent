# Gated linear recurrence: associativity and adjoints

Status: **derivation only**. No kernel exists yet. Every claim here is
checkable by hand and by finite differences on the TetherScript interpreter
before any GPU code is written.

## Mechanism

A gated linear recurrence over a matrix-valued state. For timestep `t`, key
`k_t`, value `v_t`, query `q_t`, and scalar gate `a_t`:

```text
S_t = a_t * S_{t-1} + v_t k_t^T
y_t = S_t q_t
```

`S_t` is `d_v x d_k`. This is the delta rule with decay, and it is the shared
skeleton behind gated linear attention, DeltaNet, and the selective-scan
family. Softmax attention is not expressible this way, which is the point:
the state is a fixed-size learned map rather than a growing cache.

## Why it parallelizes

Sequential evaluation is `O(T)` depth. The recurrence is affine in `S`, so
each step is a pair `(a, B)` acting as `S -> a*S + B`. Composition of two
such maps is again of that form:

```text
(a_2, B_2) . (a_1, B_1) = (a_2*a_1, a_2*B_1 + B_2)
```

Define the operator on pairs:

```text
(a_i, B_i) @ (a_j, B_j) = (a_i*a_j, a_i*B_j + B_i)
```

reading `@` as "apply j after i".

### Associativity

Let `x = (a_1, B_1)`, `y = (a_2, B_2)`, `z = (a_3, B_3)`.

```text
(x @ y) @ z = (a_1*a_2, a_1*B_2 + B_1) @ (a_3, B_3)
            = (a_1*a_2*a_3, a_1*a_2*B_3 + a_1*B_2 + B_1)

x @ (y @ z) = (a_1, B_1) @ (a_2*a_3, a_2*B_3 + B_2)
            = (a_1*a_2*a_3, a_1*(a_2*B_3 + B_2) + B_1)
            = (a_1*a_2*a_3, a_1*a_2*B_3 + a_1*B_2 + B_1)
```

Equal. Associativity holds because scalar multiplication is associative and
distributes over matrix addition. The identity is `(1, 0)`, so the pairs form
a monoid and a Blelloch scan computes all prefixes in `O(log T)` depth.

The operator is **not commutative**: `a_1*B_2 + B_1` differs from
`a_2*B_1 + B_2` in general. Scan order must therefore be preserved, which is
the constraint any tiled implementation has to respect.

## Adjoints

There is no autodiff here, so the backward pass is derived. Given upstream
gradient `dy_t`, with `S_0 = 0`:

Forward, unrolled:

```text
S_t = sum_{i<=t} (prod_{j=i+1..t} a_j) v_i k_i^T
y_t = S_t q_t
```

Gradient with respect to the query is immediate:

```text
dq_t = S_t^T dy_t
```

Gradient with respect to the state accumulates backward, because `S_t`
influences every later output:

```text
dS_t = dy_t q_t^T + a_{t+1} * dS_{t+1},    dS_{T+1} = 0
```

That is the same affine recurrence run in reverse, so the backward pass is
also a scan and reuses the same operator.

From `dS_t`, the remaining gradients follow from `S_t`'s dependence on
`v_t k_t^T` and on `S_{t-1}`:

```text
dv_t = dS_t k_t
dk_t = dS_t^T v_t
da_t = <dS_t, S_{t-1}>        (Frobenius inner product)
```

## What must be verified before a kernel is written

1. A scalar TetherScript reference implementing the sequential recurrence.
2. A scan-based TetherScript implementation using `@`, matching the reference
   to floating-point tolerance.
3. Finite-difference checks on every gradient above.

Only then does a CUDA kernel have a correctness oracle. The tree-walking
interpreter and the bytecode VM share observable semantics, so agreement
between them is an additional independent check.

## Hardware constraint

The target device is an RTX 2080 SUPER, sm_75: 48 SMs, 48 KB shared memory
per block, 64 KB per SM, fp16 tensor cores via `mma.sync.m16n8k8`. There is
no native bfloat16, no FP8, no TMA, and no asynchronous copy. Chunked scans
and fusion are viable; low-precision numerics research is not.
