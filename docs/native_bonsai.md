## Observed dedicated-provider run (dev.11)

- **Static/local, real CUDA:** the standalone `tetherscript run
  examples/tetherscript/bonsai_provider.tether` command returned
  `BONSAI_NATIVE_OK` with exit code 0 through `codetether bonsai`.
- **Static/local, real CUDA:** two direct requests in one process returned the
  same token; the second reported `load_ms: 0.0`, `prefill_ms: 5018.412587`,
  `decode_ms: 1094.604129`, and six generated token IDs. The five post-first-token
  intervals correspond to approximately **4.57 decode tokens/sec**. This small
  smoke measurement is not a large-sample performance benchmark.
- **Focused CI-like:** the three embedded TetherScript/protocol regressions ran
  with zero failures. Nine provider/native CPU tests also passed. The two
  explicitly enabled CUDA tests passed: PQ2 multiplication and Hadamard rotation
  matched their CPU references. These checks do not establish full-model logit
  parity against an independent implementation.

The current validation transcript is retained outside the checkout at
`~/.local/state/codetether/codetether-bonsai-validate-20260921T024555Z.log`.

# Native Candle: Ternary Bonsai 2 27B PQ2

## Scope and current validation

The dedicated provider is implemented in `src/provider/bonsai/`; native checkpoint
loading and CUDA execution are under `native/`. It does not construct a
`ThinkerClient`, run an agent or RLM loop, or call an HTTP model endpoint.

The binary must include `candle-cuda` and `tetherscript` (the latter is a default
feature). A CPU-only reinstall cannot execute Bonsai. The prior `dev.8` smoke
used the older adapter; it does not validate the new dedicated provider.

## Installed model data

- GGUF: `prism-ml/Ternary-Bonsai-2-27B-gguf`, revision `6ed5e12bf84b7a63069882c91dd9e9218647d17b`.
- File: `Ternary-Bonsai-2-27B-PQ2_0.gguf`, 7,206,168,928 bytes.
- SHA256: `3907dc1658db1f78a9826bf8d5bcb8dc65db0d466388937af57f2294fae62ec1`.
- Tokenizer data comes from the official companion checkpoint at revision `3f926b415992eaa2ae9dd7b573706494d6bbf787`; no MLX code is used. Its 248,044 vocabulary entries and 33 added tokens were compared against the GGUF IDs.
- Tokenizer SHA256: `0997f410c57a1f4e53b09e4be8f4a172d90edd9564368fb0847030937229b9f3`.

## Implementation contract

- A bounded GGUF reader recognizes Prism-private tensor type **142**, not Candle's unrelated Q2K format.
- Packed matrices remain PQ2_0 on CUDA; kernels decode during multiplication, without a full FP16 copy.
- The packed embedding stays in host memory; only the selected row is decoded and transferred.
- Explicit signed, normalized Hadamard transforms are applied before folded matrix projections and reversed after embedding lookup.
- The decoder implements the checkpoint's 64-layer text-only hybrid: 48 gated-delta recurrent layers and 16 full-attention layers, including the Prism grouped-value-head permutation.
- KV/recurrent state is reset between unrelated requests. Prefix reuse is disabled initially.
- Runtime context is deliberately **4096**, not the checkpoint's 262K training window. Prefill is sequential and correctness-first; performance is not yet established.
- Direct text streaming emits decoded token deltas while generation runs. Image input and structured tool calling are not advertised.
- Model weights persist across requests on one provider instance; sampling changes do not reload them. Load, prefill, TTFT, and warm decode timing are separate fields.

## Use only after the native build and parity gates succeed

```sh
. script/bonsai-candle-env.sh
codetether models --provider bonsai
codetether bonsai --prompt "Reply with exactly BONSAI_NATIVE_OK" --repeat 2
# Runnable TetherScript entrypoint (not a hooks-only file):
tetherscript run examples/tetherscript/bonsai_provider.tether -- "Say hello"
```

The direct command bypasses agent/session/Vault initialization and reads only the
local checkpoint configuration. Registry discovery uses explicit environment
opt-in, or a Vault `bonsai` entry with `model_path`, `tokenizer_path`, and optional
`cuda_ordinal`. The preset does not bypass `CODETETHER_DISABLE_ENV_FALLBACK=1`.

## Required validation before enabling

CPU reference tests cover packing, half scales, inverse rotation, delta updates and head ordering. Separate ignored CUDA parity tests compare native packed operations to those references. CI must compile the CUDA path, execute those GPU tests, and perform a bounded full-model completion on the target GPU before it is considered usable.

The format/transform reference is the MIT-licensed PrismML llama.cpp source at `d8f26eec7`; it is used as a specification, not linked or executed by this backend. Model weights remain under the publisher's Apache-2.0 model license.