# Session Types in CodeTether Agent

This document catalogs the distinct kinds of "sessions" in the system, with
particular focus on **Codex sessions**. For each type it covers how it is
**created**, **stored**, and **consumed**, plus the **data shape** produced.

Source of truth: `src/session/` (especially `src/session/codex_import/`,
`src/session/thread_store/`, `src/session/tasks/`) and
`src/cli/run/jsonl/codex_thread.rs`.

---

## 1. Native Session (the canonical session)

The primary, first-class session. Everything else either feeds into it or is a
sidecar of it.

### Type / data shape
Defined in `src/session/types.rs`:

- `Session`
  - `id: String` — UUID, used as the on-disk filename.
  - `title: Option<String>` — auto-generated from the first user message.
  - `created_at: DateTime<Utc>`, `updated_at: DateTime<Utc>`.
  - `metadata: SessionMetadata` (serialized **before** `messages` so the
    `SessionHeader` prefilter can match a workspace cheaply).
  - `agent: String` — persona name that owns the session (e.g. `"build"`).
  - `messages: Vec<Message>` — ordered conversation transcript.
  - `pages: Vec<PageKind>` — per-message history/context classification sidecar
    (backfilled for legacy sessions).
  - `summary_index: SummaryIndex` — hierarchical summary cache (lazy).
  - `tool_uses: Vec<ToolUse>` — per-tool-call audit records.
  - `usage: Usage` — aggregate token usage.
  - `max_steps: Option<usize>` (`#[serde(skip)]`), `bus: Option<Arc<AgentBus>>`
    (`#[serde(skip)]`) — runtime-only.

- `SessionMetadata`: `directory`, `model` (`"provider/model"`),
  `knowledge_snapshot`, `provenance: Option<ExecutionProvenance>`,
  `auto_apply_edits`, `allow_network`, `slash_autocomplete`, `use_worktree`,
  `shared`, `share_url`, RLM settings, `history_sink`, plus more.

### Creation
- `Session::new(...)` during a TUI/CLI/worker run; messages appended via
  `add_user_message` / `add_message`.

### Storage
- `src/session/persistence.rs`: one JSON file per session under the data dir's
  `sessions/` folder, named `<id>.json`.
- Written as **compact** JSON via `serde_json::to_vec` on the blocking pool.
- **Atomic** writes: temp file + rename (`<id>.json.tmp` → `<id>.json`).
- Load with `Session::load(id)`; size-capped sanity check before parse.
- Workspace-scoped lookup via `last_for_directory[_tail]` using `SessionHeader`
  prefilter (matches `metadata.directory` without lexing the transcript).

### Consumption
- Resumed in the TUI/CLI; scanned by directory; serves as the import target for
  Codex sessions.

---

## 2. Codex Session (imported from OpenAI Codex CLI rollouts)

A **Codex session** is an external rollout produced by the OpenAI Codex CLI,
stored under the Codex home directory. CodeTether **discovers, parses, and
imports** these into Native Sessions. It does not author them.

### Where they live (storage / source)
- Codex home: `$CODEX_HOME`, else `$HOME/.codex` (`paths.rs::codex_home_dir`).
- Rollouts: `<codex_home>/sessions/**/*.jsonl` (recursively walked).
- Title index: `<codex_home>/session_index.jsonl`
  (entries: `{id, thread_name, updated_at}`; latest `updated_at` wins).
- Files are **JSON Lines** (`.jsonl`), one record per line.

### On-disk record formats (data shape)
Two formats are accepted (`legacy.rs`, `records.rs`):

**Modern envelope** — every line is:
```json
{ "timestamp": "<RFC3339>", "type": "<kind>", "payload": { ... } }
```
where `type` ∈ `session_meta`, `turn_context`, `response_item`, `event_msg`.

**Legacy flat** — first line is the meta itself (`{id, timestamp,
instructions, git}`, **no** `type`, **no** `cwd`); subsequent lines are bare
`{type: "message"|"reasoning"|"function_call"|...}` response items; a
`{record_type: "state"}` marker is ignored. `normalize_line` transcribes legacy
lines into the modern `CodexJsonlRecord` envelope so the rest of the pipeline is
format-agnostic.

Key payload structs (`records.rs`, `payloads.rs`):
- `CodexSessionMetaPayload`: `{ id, timestamp, cwd }`.
- `CodexTurnContextPayload`: `{ model? }`.
- `CodexResponseMessagePayload`: `{ role, content: [CodexContentItem] }` where
  each item is `{ type, text?, image_url? }` with type in
  `input_text|output_text|input_image`.
- `CodexFunctionCallPayload`: `{ name, arguments, call_id?, id? }`.
- `CodexFunctionCallOutputPayload`: `{ call_id, output }`.
- `CodexReasoningPayload`: `{ summary: [..], content? }`.
- `CodexTokenEnvelope` → `info.total_token_usage`:
  `{ input_tokens, cached_input_tokens?, output_tokens, total_tokens }`
  (only consumed from `event_msg` records of type `token_count`).

### Creation (import / parse pipeline)
Entry points in `codex_import/api.rs`:
- `import_codex_session_by_id(id)` — find rollout by id, import.
- `import_codex_session_path(path)` — import a specific rollout file.
- `import_codex_sessions_for_directory(dir)` — import all rollouts whose `cwd`
  matches `dir`; returns `CodexImportReport { imported, skipped }`.
- `load_or_import_session(id)` — prefer Codex re-import if present, else load
  existing native session.

CLI surface: `src/cli/run.rs` accepts `--codex-session <id>` and calls
`import_codex_session_by_id`.

Parsing (`parse.rs::parse_codex_session_from_path`) maps records → a native
`Session`:
- `session_meta` → `id`, `created_at` (= meta.timestamp), `directory` (= `cwd`,
  canonicalized; legacy `cwd` recovered from the first `<environment_context>`
  user message via `legacy.rs::extract_cwd_from_env_context`).
- `turn_context.model` → `metadata.model`.
- `response_item` → `Message`s (`response/`):
  - `message` → `Role::User`/`Assistant` with `Text`/`Image` parts.
  - `function_call` → `Role::Assistant` + `ContentPart::ToolCall`.
  - `function_call_output` → `Role::Tool` + `ContentPart::ToolResult`.
  - `reasoning` → `Role::Assistant` + `ContentPart::Thinking`.
- `event_msg` (type `token_count`) → `usage`.
- `updated_at` = max record timestamp.
- `title` = override (from `session_index.jsonl`) else first user line
  (truncated to 60 chars).

Memory safety: `budget.rs::BoundedMessages` caps imported messages at
`MAX_IMPORTED_MESSAGES = 4000` (FIFO eviction, logs dropped count) because
rollouts can be hundreds of MB.

### Storage (after import) / consumption
- `persist.rs::persist_imported_session` writes the converted `Session` to the
  **native** sessions dir as `<id>.json` (pretty JSON here), but **skips** the
  write when an existing native session has `updated_at >= imported.updated_at`
  (→ `PersistOutcome::Unchanged`).
- `discover.rs` + `summary.rs` produce `CodexSessionInfo`
  (`{ id, path, title, created_at, updated_at, message_count, agent,
  directory }`) for listing without full parse.
- After persistence, callers `Session::load(id)` to consume as a normal native
  session.

### How Codex sessions differ from generic/native sessions
| Aspect | Native Session | Codex Session (source) |
|---|---|---|
| Format | single `<id>.json` (compact JSON) | `.jsonl`, one record/line |
| Origin | authored by this agent | external OpenAI Codex CLI rollout |
| Location | data dir `sessions/` | `$CODEX_HOME/sessions/**` |
| Envelope | one `Session` object | `{timestamp,type,payload}` per line (+ legacy flat) |
| Title source | first user message / generated | `session_index.jsonl` `thread_name`, else first user line |
| `cwd`/dir | `metadata.directory` field | meta `cwd`, or recovered from `<environment_context>` |
| Usage | accumulated live | parsed from `event_msg` `token_count` |
| `agent` | persona that created it | hardcoded `"build"` on import |
| Sidecars | `pages`, `summary_index`, `tool_uses` populated | empty on import (`pages=[]`, `tool_uses=[]`) |
| Size guard | full transcript kept | capped at 4000 messages (FIFO) |

---

## 3. Codex-shaped Thread JSONL (export adapter)

The inverse direction: durable thread events written **out** in a Codex-shaped
JSONL envelope. `src/cli/run/jsonl/codex_thread.rs`.

- `write_codex_thread_event_to` serializes a `ThreadEvent` as
  `{ timestamp, type, payload }` (RFC3339 millis timestamp).
- The payload is the event's JSON object augmented with `event_id`,
  `thread_id`, `session_id`, `turn_id`.
- This mirrors the modern Codex envelope so other tooling can consume our thread
  stream the same way it consumes Codex rollouts.

---

## 4. Thread Event Store (append-only thread/turn events)

`src/session/thread_store/`. Append-only event log, distinct from the native
session JSON.

- Type `ThreadEvent` (`types.rs`): `{ event_id, thread_id, session_id,
  turn_id, kind, timestamp_ms, payload }`.
- Storage: one JSONL file per thread, `<thread-id>.jsonl`, under a root dir
  (`ThreadStore`). Thread ids validated (alphanumeric/`-`/`_`, ≤128 chars) to
  prevent path traversal (`path.rs`).
- Created via `ThreadStore` append; consumed by readers and by the
  codex-thread export adapter above.

---

## 5. Session Task Log (per-session task/goal state)

`src/session/tasks/`. Tracks goals and task items for a session.

- Storage: `.tasks.jsonl` per session (`path.rs`), append-only event log
  (`log.rs`), with derived `state.rs` and `render.rs`.
- Exposed through the `session_task` tool (`set_goal`, `task_add`,
  `task_status`, etc.).

---

## 6. Other session-adjacent stores

These are not "conversation sessions" but live under `src/session/` and shape
session behavior:

- **Checkpoints** (`checkpoint_*.rs`): point-in-time conversation snapshots used
  for compaction/restore.
- **History sink** (`history_sink.rs`, `history*.rs`): external streaming of
  history records.
- **Eval / oracle** (`eval/`, `oracle*.rs`): evaluation and replay harnesses.
- **Index / workspace index** (`index/`, `workspace_index*.rs`): summary and
  workspace lookup caches.

---

## Summary

- The **Native `Session`** (`<id>.json`) is the canonical conversation object.
- **Codex sessions** are external `.jsonl` rollouts under `$CODEX_HOME/sessions`
  in either a modern `{timestamp,type,payload}` envelope or a legacy flat
  schema; they are discovered, parsed, and imported (capped at 4000 messages)
  into native sessions, deriving `directory`/`model`/`usage`/`title` from
  rollout records and the `session_index.jsonl`.
- A **Codex-shaped export adapter** writes our `ThreadEvent`s back out in the
  same envelope.
- The **ThreadStore** and **Session Task Log** are append-only JSONL sidecars
  keyed by thread/session id, distinct from the single-file native session.
