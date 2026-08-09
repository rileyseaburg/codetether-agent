# Ergonomic issues backlog

Observed while implementing mux paste handling, symbol-search responsiveness,
and the native `rg` tool. These are runtime/tooling defects, not hypotheses.

## Collaboration and worktree leases

- A parent waiting on a child retains the exclusive worktree write lease. A
  child sharing that workspace cannot edit, producing a parent/child deadlock.
- The blocked lease is continuously renewed, so waiting does not recover.
- `agent spawn` rejects delegation from mux sessions while `spawn_agent`
  accepts it, exposing inconsistent collaboration policy.
- A child can return its requested result but be marked failed solely because
  it omitted a harness-specific terminal status line.
- `list_agents` can fail before execution because its empty argument object is
  reported as truncated by the provider output-token limit.

## Editing and diagnostics

- `edit` with `replace_all: true` can still fail `AMBIGUOUS_MATCH` in the
  confirmation layer.
- The stub-marker guard treats legitimate identifiers such as
  `todo::TodoReadTool` as placeholders and rejects the edit.
- Patch preapproval can use stale LSP diagnostics after the file on disk has
  already been corrected.
- Shell policy blocks scratch-file here-docs and `sed -i`, including work in
  temporary directories unrelated to repository edits.

## Search and output fidelity

- The legacy `grep` tool escapes regex metacharacters unless callers remember
  `is_regex: true`; `a|b` otherwise produces a silent false negative. The new
  `rg` tool avoids this opt-in trap.
- Tool output has silently substituted substrings such as `glm` to `il`,
  `retry` to `ln`, and `bracketed` to `n`. Direct file reads showed the source
  was intact, making this an output-rendering or transport defect.

## TUI runtime cost

- One live TUI measured 339 MiB RSS, with 315 MiB anonymous private memory and
  a 630 MiB high-water mark. The allocator watchdog previously waited until
  1,024 MiB before calling `malloc_trim`, retaining large transient buffers.
- Mux status reporting cloned several owned strings on every event-loop pass
  before checking whether the status had changed.
- Server-side Ctrl+V clipboard reads use the detached TUI process's display
  environment, not the mux client's Linux clipboard. Clipboard resolution now
  belongs at the mux input boundary.

## Provider routing

- The Bedrock alias table resolves Opus 5 to
  `global.anthropic.claude-opus-5`, while the requested
  `us.anthropic.claude-opus-5` identifier passes through unchanged. The probe
  returned model output, but the child harness classified the run as failed
  because of the missing terminal status line, so exact resolved-model
  telemetry remains incomplete.