# Persistent goals

CodeTether goals preserve a user-authored objective across model turns. An
active goal is stored beside the session as append-only JSONL, injected into
the system prompt, and continued automatically when the model would otherwise
finish.

## TUI commands

```text
/goal set <objective>
/goal show
/goal edit <objective>
/goal reaffirm <progress note>
/goal pause
/goal resume
/goal done
/goal clear [reason]
```

`done` preserves final accounting with status `complete`; `clear` removes the
goal from the folded session state. Paused, complete, blocked, usage-limited,
and budget-limited goals do not continue automatically.

## Model tools

- `create_goal` creates an explicitly requested goal and optional positive
  token budget. It rejects a new goal while an unfinished goal exists.
- `get_goal` returns lifecycle status, token/time use, turns, and remaining
  budget.
- `update_goal` accepts only `complete` or `blocked`. Completion requires a
  requirement-by-requirement audit; blocked requires the same external blocker
  on three consecutive goal turns.

Provider token use and elapsed call time are appended after every successful
model response. A protocol-level `Done`/`response.completed` event finishes the
turn immediately without waiting for the transport connection to close. A
connection that closes or stalls before that terminal event is incomplete: its
partial preview is abandoned and the whole sampling request is retried rather
than token-stitched. Reconnects use Codex-style jittered exponential backoff
starting near 200 ms. Provider rate-limit guidance such as `try again in
1.898s` overrides that backoff at both the transport and whole-request layers.
WebSocket fallback state and reusable connections are isolated by session, so
one interrupted thread cannot disable or inherit transport state from another.
If retryable stream recovery is exhausted, an active goal gets up to two
whole-turn continuations. A third consecutive failure in the same category
stops the loop as blocked, or `usage_limited` for rate limits. Permanent
provider errors are returned immediately.

Goal continuation remains subordinate to user steering: queued user input is
drained before an automatic continuation is scheduled.

On TUI startup or session selection, an idle session with an active persisted
goal starts that continuation automatically. `/continue` remains the manual
recovery path for a deliberately interrupted or stalled turn.
