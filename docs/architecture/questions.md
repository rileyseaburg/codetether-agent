# Interactive questions

`crates/codetether-question` owns the question contract, input validation,
selection state, and async question/answer rendezvous. It does not depend on the
agent, terminal, or provider crates. The agent's `question` tool is a thin adapter.

Internally, `request/` separates wire models, legacy decoding, validation, and
read access. `draft/` owns private selection state and separates navigation/editing
from answer commitment. `runtime/` separates task-local capability, pending-state
storage and cleanup, asynchronous waiting, and frontend controls. These modules
retain the same public exports; the tool and TUI do not own copies of their logic.

## Upstream reference

The implementation follows the interaction in the local OpenCode checkout:

- `.upstream/opencode/packages/schema/src/question.ts`: question/header,
  labeled options with descriptions, optional multiple selection, answer arrays.
- `.upstream/opencode/packages/core/src/tool/plugin/question.ts`: nonempty batches,
  automatic custom answers, and ordered answer lists.

This is a native Rust implementation, not a dependency on the TypeScript code.
Unlike OpenCode's generic form service, this crate provides a question-specific,
process-local rendezvous. Pending questions are not persisted across restarts.

## Model-facing input

```json
{
  "questions": [{
    "question": "Which implementation should I use?",
    "header": "Approach",
    "options": [
      {"label": "Small patch (Recommended)", "description": "Preserve the current design"},
      {"label": "Refactor", "description": "Change the surrounding structure"}
    ],
    "multiple": false
  }]
}
```

The tool returns `{"answers":[["Small patch (Recommended)"]]}` and the same
answer lists in metadata. Legacy `question`, string `options`, and `default`
arguments are accepted; a legacy default is placed first, never auto-submitted.
Custom text is always available. Empty submissions leave the panel open.

## Runtime and frontend boundaries

The TUI's foreground `run_prompt` wraps the session in `runtime::scope`.
Only that async task advertises and executes interactive questions. Spawned
agents do not inherit this Tokio task-local capability. Existing autonomous
tool filtering remains intact. CLI `run`, workers, servers, and other headless
interfaces continue to exclude questions and reject direct calls without
reading stdin. A future frontend can opt in by supplying the same scope and
driving `pending`, `act`, and `dismiss` for its exact session ID.

`runtime::ask` publishes one pending batch per session and awaits a oneshot.
Runtime-injected session IDs must match the attached scope. Different sessions
cannot receive each other's answers. Dropping the waiting future removes its
pending state, including turn cancellation and panic unwinding. Escape dismisses
the question and returns explicit model-facing failure feedback; unlike
OpenCode's silent-dismissal path, this does not terminate the whole turn.
Ctrl+C retains CodeTether's existing whole-turn cancellation behavior.

The TUI polls pending state on its normal tick, renders a modal choice list,
and consumes answer keys and bracketed paste before the chat composer. The
watchdog treats an open question as human wait time, not a provider stall.

### Keys

- Up/Down or Tab: move through choices and the custom-answer row.
- Space: toggle a choice for multi-select.
- Type or paste: enter a custom answer.
- Enter: submit the current answer; finish after the final question.
- Shift+Tab: return to the previous question with selections preserved.
- Escape: dismiss without supplying an answer.
- Ctrl+C / Ctrl+Q: existing turn-cancel / quit behavior.

## Focused checks

```sh
cargo test -p codetether-question
cargo clippy -p codetether-question --all-targets -- -D warnings
cargo test --lib question
./check_file_limits.sh
```

The crate tests cover validation, legacy input, batching, single/multi/custom
answers, isolation, headless behavior, dismissal, and cancellation cleanup.
Agent tests exercise the actual tool adapter, catalog filtering, key/paste
handling, watchdog suppression, and in-memory ratatui rendering. In-memory
rendering is not a live-terminal or real-provider interaction test.
