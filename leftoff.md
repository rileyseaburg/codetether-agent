# Left Off - Oracle FINAL Enforcement + Local CUDA E2E

Date: 2026-02-24
Branch: `feature/copilot-review-fixes-pr2`

## What was implemented

### 1) Strict FINAL(JSON) enforcement in RLM

Updated `src/rlm/repl.rs` so the RLM system prompt now explicitly requires:
- final response must be exactly `FINAL(<json>)`
- no prose final answers
- payload must match one of:
  - `kind=grep`
  - `kind=ast`
  - `kind=semantic` (only when deterministic payload is not possible)

Also added a concrete `FINAL({...})` example in prompt text.

### 2) Fallback output coercion (safety net)

Added executor-side fallback in `src/rlm/repl.rs`:
- `ensure_structured_final_payload(...)`
- if final answer is malformed/prose, try coercing from trace history
- for pattern-match queries, build `FinalPayload::Grep` from recent grep-style trace outputs
- preserves oracle-eligibility even when model formatting slips

Helper methods added:
- `coerce_grep_payload_from_trace(...)`
- `extract_latest_grep_matches(...)`
- `parse_line_numbered_output(...)`
- `infer_file_from_query(...)`

### 3) `rlm_final` tool schema updated for structured payloads

Updated `src/rlm/tools.rs`:
- `rlm_final` now prefers `payload` object (JSON)
- keeps `answer` as compatibility fallback
- dispatcher returns stringified JSON when `payload` object is provided

## Tests run

- `cargo fmt`
- `cargo test rlm::repl -- --nocapture`
- `cargo test rlm::tools::tests::tool_definitions_are_complete -- --nocapture`

All passed.

## Installed-binary CUDA E2E run

Installed binary:
```bash
cargo install --path . --force --features candle-cuda,functiongemma
```

GPU on host:
- `NVIDIA GeForce RTX 2080 SUPER` (8GB)

Downloaded model/tokenizer for local run:
- `Qwen/Qwen3-4B-GGUF` -> `Qwen3-4B-Q4_K_M.gguf`
- `Qwen/Qwen3-4B` -> `tokenizer.json`

Executed:
```bash
export LOCAL_CUDA_MODEL_PATH=/home/riley/models/qwen3-4b/Qwen3-4B-Q4_K_M.gguf
export LOCAL_CUDA_TOKENIZER_PATH=/home/riley/models/qwen3-4b/tokenizer.json
codetether rlm --model local_cuda/qwen3-4b --file src/rlm/repl.rs --json \
  "Find all occurrences of 'async fn' in src/rlm/repl.rs"
```

Observed:
- provider: `local_cuda`
- model: `qwen3-4b`
- final output is now structured JSON (`kind=grep`) instead of malformed/prose
- oracle verdict: `failed` (payload content mismatch vs ground truth), not parse/malformed failure

## TPS measurement

Measured from CLI JSON stats (`output_tokens / (elapsed_ms / 1000)`):
- `output_tokens = 333`
- `elapsed_ms = 8396`
- **TPS = 39.66 tokens/sec**

## Additional observation

- `Qwen2.5-Coder-0.5B q4_k_m` failed in local CUDA sampling on this setup:
  - `A weight is negative, too large or not a valid number`
- `Qwen3-4B q4_k_m` runs correctly and is the better baseline for local E2E.

## Current bottleneck

Formatting bottleneck is fixed (FINAL payload structure is emitted).
Remaining bottleneck is answer quality/completeness for deterministic grep truth (oracle still `failed` on this specific run).

## Files changed in current workstream

- `src/cognition/thinker.rs`
- `src/main.rs`
- `src/provider/local_cuda.rs`
- `src/rlm/repl.rs`
- `src/rlm/tools.rs`
- `tests/rlm_provider_resolution.rs`
- `leftoff.md`
