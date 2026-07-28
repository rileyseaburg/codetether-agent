"""Base models pinned to what the deployment target can actually run.

The trained model is served on an NVIDIA RTX 2080 Super with 8 GB of VRAM,
so the deployment constraint, not the training constraint, decides size.
Measured at Q4_K_M with an 8,192 token KV cache:

| model | weights | total | fits 8 GB |
|---|---:|---:|---|
| Qwen3.6-27B | 15.3 GB | 16.3 GB | no |
| Qwen3-Coder-30B-A3B | 16.8 GB | 17.8 GB | no |
| Qwen3.5-9B | 5.3 GB | 7.0 GB | yes |
| Qwen3.5-4B | 2.6 GB | 4.3 GB | yes |

Turing hardware also lacks bfloat16, so inference runs in float16.
"""

BASE_MODEL = 'Qwen/Qwen3.5-9B'
BASE_REVISION = 'c202236235762e1c871ad0ccb60c8ee5ba337b9a'
"""Qwen3.5-9B: 9.7B dense parameters, 19.3 GB of bf16 weights.

The largest model that serves on an 8 GB card at Q4_K_M with a useful
context window. Its chat template emits `<tool_call>` and accepts a `tools`
variable, which the rendered corpus depends on.

Chosen over `microsoft/Fara1.5-9B`, which shares the `qwen3_5` architecture
but is a multimodal browser computer-use agent driven by screenshots, so its
post-training does not match a text coding corpus.
"""

SMALL_MODEL = 'Qwen/Qwen3.5-4B'
SMALL_REVISION = '851bf6e806efd8d0a36b00ddf55e13ccb7b8cd0a'
"""Qwen3.5-4B: 4.7B parameters, 9.3 GB, same `qwen3_5` architecture.

Used for proving runs so architecture support is validated once for both
tiers, and for devices that cannot hold the 9B.
"""
