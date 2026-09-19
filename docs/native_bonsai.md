# Native Candle: Ternary Bonsai 2 27B PQ2

## Scope and current validation

This is a **source implementation**, not a claim that the installed binary can run it. No Cargo compilation, Rust tests, GPU parity tests, or native full-model inference were run after the user's stop instruction. The earlier llama service was stopped and disabled; its smoke result is **not** evidence for Candle.

The native path uses CodeTether's existing `local_cuda` provider and Candle thinker. It never launches llama.cpp, Python, or an inference HTTP server. The built binary must include `candle-cuda`; the normal CPU-only release is not sufficient.

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
- Image input, structured tool calling, and streaming are **not advertised** by this existing native provider. A general text-inference model is not yet a replacement for the tool-using build agent.

## Use only after the native build and parity gates succeed

```sh
. script/bonsai-candle-env.sh
codetether models --provider local_cuda
codetether run --model local_cuda/ternary-bonsai-2-27b-pq2 --agent plan "Reply briefly."
```

The preset does not change your global default, start a server, or enable environment fallback in a Vault-only deployment. If `CODETETHER_DISABLE_ENV_FALLBACK=1` is set, an administrator must explicitly configure the existing native provider path; the preset does not bypass that policy.

## Required validation before enabling

CPU reference tests cover packing, half scales, inverse rotation, delta updates and head ordering. Separate ignored CUDA parity tests compare native packed operations to those references. CI must compile the CUDA path, execute those GPU tests, and perform a bounded full-model completion on the target GPU before it is considered usable.

The format/transform reference is the MIT-licensed PrismML llama.cpp source at `d8f26eec7`; it is used as a specification, not linked or executed by this backend. Model weights remain under the publisher's Apache-2.0 model license.