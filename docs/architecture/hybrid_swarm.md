# CodeTether Hybrid Swarm: Oracle-First Architecture

This document is the current architecture source of truth for `codetether rlm` and oracle validation.

## Objective

Run model-driven code navigation with deterministic verification always on, while persisting full validation traces for reproducible training data.

## Locked Runtime Decisions

1. `codetether rlm` is Oracle-First.
2. Oracle classification runs by default on every RLM invocation.
3. Oracle verdicts are non-blocking for process exit (`failed` verdict does not force non-zero exit).
4. Static no-provider fallback is removed from `rlm`; provider resolution is fail-fast.
5. Provider strategy is local-first (`local_cuda`), then OpenRouter fallback when no explicit provider is pinned.
6. Default cloud model baseline is Qwen 3.5 coder family (`qwen/qwen3.5-coder-7b`) unless overridden.
7. Trace persistence writes full envelope records for all verdicts (`golden`, `failed`, `unverified`, `consensus`).
8. Remote persistence target defaults to MinIO/S3 using Bus S3 credential resolution.
9. Remote outages do not drop traces; traces spool locally and can be synced later.

## `rlm` Command Flow

1. Build source content from `--file` or `--content`.
2. Resolve provider/model:
   - Explicit `--model provider/model`: use requested provider or fail.
   - Explicit `local_cuda/...` without runtime config fails (no silent fallback).
   - Unqualified model: try `local_cuda`, else `openrouter`.
   - No model: try `local_cuda` with local default, else `openrouter` with default Qwen model.
3. Execute RLM analysis.
4. Validate result through oracle (unless explicitly disabled with `--no-oracle-verify`).
5. Persist full oracle envelope via spool-first storage pipeline.
6. Print answer and oracle status.

## Oracle CLI

### `codetether oracle validate`

- Deterministically validates a provided FINAL payload against source.
- Supports `--file` or `--content`, and `--payload` or `--payload-file`.
- JSON mode emits verdict + full record envelope.
- Persistence is opt-in with `--persist`.

### `codetether oracle sync`

- Flushes local pending spool records to configured MinIO/S3 target.
- Reports `uploaded`, `retained`, `failed`, and `pending_after`.

## Persistence Semantics

### Remote config resolution

Uses Bus S3 resolution order:

1. `MINIO_*` / `CODETETHER_BUS_S3_*`
2. `CODETETHER_CHAT_SYNC_MINIO_*`
3. Vault `chat-sync-minio`

### Local spool

- Default: `~/.codetether/traces/pending/`
- Override: `CODETETHER_ORACLE_SPOOL_DIR`
- Writes are atomic (temp file then rename).

### Upload behavior

1. Write local spool file first.
2. Attempt remote upload.
3. On success, remove local spool file.
4. On failure, keep local spool file and report warning.

## Output Contracts

### Non-JSON `codetether rlm`

- Prints normal answer.
- Always appends one oracle status line:
  - `[oracle: golden ✓] ...`
  - `[oracle: failed ✗] ...`
  - `[oracle: unverified —] ...`
  - `[oracle: consensus ✓] ...`

### JSON `codetether rlm`

- Always includes `oracle` object with verdict metadata and persistence state.

## Compatibility Notes

- `--oracle-verify` remains accepted for compatibility but is deprecated and no-op.
- Use `--no-oracle-verify` for explicit emergency opt-out.

