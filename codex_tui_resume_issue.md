@codetether

## Problem

TUI Codex session loading/resume has a few correctness and performance gaps.

### 1. Startup resume ignores unimported Codex sessions

TUI startup resumes only native CodeTether session JSON via `Session::last_for_directory_tail(Some(&cwd), ...)` in `src/tui/app/run.rs`.

That path scans `Config::data_dir()/sessions/*.json` and does not consider Codex JSONL files under `~/.codex/sessions`.

If the latest session for a workspace is a Codex session that has not already been imported, TUI startup starts a new CodeTether session instead of resuming the Codex session.

### 2. Selecting an already-imported Codex session can load stale native data

`src/tui/app/session_loader.rs` tries native tail load first:

```rust
match Session::load_tail(id, session_resume_window()).await {
    Ok(load) => ...,
    Err(native_error) => load_codex(id, native_error).await,
}
```

If a Codex session was previously imported, this native load succeeds and never calls `load_or_import_session`.

That means if the original Codex JSONL continued changing externally, selecting the session in TUI can load the stale native copy even though `load_or_import_session` is designed to prefer the Codex source when it exists.

### 3. Codex import bypasses TUI tail-load safeguards

The native path uses `Session::load_tail(id, session_resume_window())`, reports dropped entries and file size, and can fork a truncated session to protect the full on-disk history.

The Codex path calls `load_or_import_session`, which parses the full Codex JSONL and returns `dropped = 0`, `file_bytes = 0`.

For very large Codex sessions, this can be slow or memory-heavy and does not trigger the TUI large-session protection/status path.

### 4. `/import-codex` double-parses matching sessions and can block the TUI

Discovery parses matching Codex JSONL files to build session info in `src/session/codex_import/discover.rs`.

Then import parses them again in `src/session/codex_import/api.rs` before persisting.

For large Codex histories, `/import-codex` can do unnecessary work from the TUI command path.

### 5. Imported Codex sessions are persisted with pretty JSON

`src/session/codex_import/persist.rs` uses `serde_json::to_string_pretty`, while normal `Session::save` intentionally uses compact JSON for performance.

Imported Codex sessions are therefore larger and slower to scan/load than regular CodeTether sessions.

## Expected behavior

- TUI startup resume should consider the newest workspace session across both native CodeTether sessions and Codex sessions.
- Selecting a Codex session should not load a stale native imported copy when a fresher Codex JSONL source exists.
- Codex resume/import should respect the same large-session/tail-window safeguards as native resume where practical.
- `/import-codex` should avoid avoidable double parsing and should not noticeably block TUI responsiveness.
- Persisted imported Codex sessions should use the same compact storage format as normal session saves.

## Suggested fix direction

- Add a merged “latest workspace session” loader for TUI startup that can compare native and Codex summaries.
- Update `load_session_for_tui` to detect Codex-backed ids and import/refresh before falling back to native, or compare timestamps before choosing.
- Add a tail-capped Codex import/load path or avoid full import during interactive resume.
- Reuse parsed discovery results where possible during `/import-codex`, or move heavy import work off the UI path.
- Replace `serde_json::to_string_pretty` in Codex persistence with compact serialization or route through the normal session save path.

## Relevant files

- `src/tui/app/run.rs`
- `src/tui/app/session_loader.rs`
- `src/tui/app/codex_sessions.rs`
- `src/tui/app/session_sync.rs`
- `src/session/listing_all.rs`
- `src/session/codex_import/api.rs`
- `src/session/codex_import/discover.rs`
- `src/session/codex_import/parse.rs`
- `src/session/codex_import/persist.rs`
- `src/session/persistence.rs`
