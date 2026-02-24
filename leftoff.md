# Left Off - Oracle FINAL Enforcement + Local CUDA Golden Run

Date: 2026-02-24
Branch: `feature/copilot-review-fixes-pr2`

## Current state

- Oracle pipeline is end-to-end and active.
- Local CUDA run now reached **golden** on deterministic grep validation.
- FINAL output is enforced as JSON and normalized from trace evidence when needed.

## What changed after the previous note

### 1) Added hard gate: no FINAL for grep queries without evidence

In `src/rlm/repl.rs`:
- pattern-match queries now reject FINAL unless grep evidence exists in trace/output.
- rejection is recorded as `reject_final(no_grep_evidence)`.

This prevents model guesses/hallucinated line numbers from being accepted as final output.

### 2) Added canonical normalization from trace evidence

In `src/rlm/repl.rs`:
- `ensure_structured_final_payload(...)` now normalizes even valid `kind=grep` payloads when model payload differs from trace-derived ground data.
- normalization step is recorded as `normalize_final_payload(grep_trace)`.

This removes model reformatting errors from the final payload path.

### 3) Fixed trace fidelity for grep normalization

In `src/rlm/repl.rs`:
- line-numbered grep outputs are no longer truncated when persisted in trace steps.
- `parse_line_numbered_output(...)` now preserves leading whitespace in line text (critical for exact oracle comparisons).

Without this, normalization could still fail on partial/truncated text.

## Local CUDA E2E results

GPU:
- `NVIDIA GeForce RTX 2080 SUPER` (8GB)

### A) Qwen3-4B (earlier run)

Command:
```bash
export LOCAL_CUDA_MODEL_PATH=/home/riley/models/qwen3-4b/Qwen3-4B-Q4_K_M.gguf
export LOCAL_CUDA_TOKENIZER_PATH=/home/riley/models/qwen3-4b/tokenizer.json
codetether rlm --model local_cuda/qwen3-4b --file src/rlm/repl.rs --json \
  "Find all occurrences of 'async fn' in src/rlm/repl.rs"
```

Observed (earlier installed-binary run):
- provider `local_cuda`
- structured `kind=grep` payload emitted
- oracle `failed` (content mismatch)
- TPS: `39.66`

### B) Qwen2.5-Coder-7B (latest source run, after fixes)

Command:
```bash
export LOCAL_CUDA_MODEL=qwen2.5-coder-7b-q4_k_m
export LOCAL_CUDA_MODEL_PATH=/home/riley/.local/share/codetether/models/qwen2.5-coder-7b-q4_k_m/qwen2.5-coder-7b-instruct-q4_k_m.gguf
export LOCAL_CUDA_TOKENIZER_PATH=/home/riley/.local/share/codetether/models/qwen2.5-coder-7b-q4_k_m/tokenizer.json
cargo run --features candle-cuda,functiongemma -- \
  rlm --model local_cuda/qwen2.5-coder-7b-q4_k_m --file src/rlm/repl.rs --json \
  "Find all occurrences of 'async fn' in src/rlm/repl.rs"
```

Observed (latest):
- provider `local_cuda`
- trace includes `grep("async fn")`
- trace includes `normalize_final_payload(grep_trace)`
- oracle verdict: **`golden`**
- TPS: `21.85`

## Additional notes

- `Qwen2.5-Coder-0.5B q4_k_m` remains unstable on this setup (`A weight is negative, too large or not a valid number`).
- `README.md` now includes exact local CUDA invocation commands for users.

## Validation/tests run

- `cargo fmt`
- `cargo test rlm::repl -- --nocapture`
- `cargo test rlm::repl::tests::test_parse_line_numbered_output -- --nocapture`
- `cargo test rlm::tools::tests::tool_definitions_are_complete -- --nocapture`

## Files touched in current workspace

- `README.md`
- `src/rlm/repl.rs`
- `src/rlm/tools.rs`
- `src/cognition/thinker.rs`
- `src/main.rs`
- `src/provider/local_cuda.rs`
- `tests/rlm_provider_resolution.rs`
- `leftoff.md`
