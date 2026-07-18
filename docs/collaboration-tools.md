# First-class collaboration tools

The model-facing collaboration surface wraps the existing durable sub-agent
runtime with focused tools:

- `spawn_agent` starts a background child and returns its durable `agent_id`
  plus canonical `task_name` path.
- `send_input` routes by `agent_id`, optionally interrupting the active turn
  before the new input is queued, and returns the durable mailbox receipt as
  `submission_id`.
- `send_message` steers a running child or records context for its next turn,
  but never wakes an idle child.
- `followup_task` steers a running child at its next model boundary and starts
  a durable background turn when the child is idle.
- `list_agents` lists the live root-thread tree, including `/root`, and accepts
  an optional absolute or caller-relative task-path prefix.
- `wait_agent` waits for any child mailbox update or steered user input; target
  lists remain accepted only for compatibility with the older status API.
- `interrupt_agent` aborts the active turn without deleting the child session.
- `close_agent` shuts down a child and frees its open-thread slot while keeping
  its transcript and mailbox.
- `resume_agent` reopens that durable child by session ID; calling it for an
  already-open child succeeds without creating a duplicate.

Child identity manifests and queued follow-ups are stored under
`<data_dir>/agents`. On startup or session selection, CodeTether reloads the
parent's children and resumes pending work. Registry entries, execution guards,
notifications, live traces, and mailboxes are all keyed by the durable child
session ID; nicknames are display metadata rather than runtime identity. A
follow-up claimed before a crash
is reset to pending on recovery, providing at-least-once delivery; the task
should therefore tolerate a repeated turn if the process died after external
side effects but before acknowledgement.

First-class task names use lowercase letters, digits, and underscores so every
child has an unambiguous `/root/...` path. `fork_turns` defaults to `all` and
also accepts `none` or a positive turn count. Canonical paths returned by
`spawn_agent` and `list_agents` resolve across the current root tree for live
messaging and for close/resume of durable unloaded descendants; direct child
names and session IDs remain supported.

`send_input` accepts either a plain `message` or Codex-style structured
`items`. Text, image data URLs, and local images are preserved in the durable
mailbox; skill and mention items are rendered into the child prompt. Audio item
shapes are recognized but rejected explicitly because the local provider
interface does not yet expose audio content.

`send_message` and `followup_task` share the live steering path but intentionally
differ when the target is idle. Queue-only messages are appended and saved in
the child transcript without acquiring an execution slot. Follow-up tasks use
the durable turn mailbox, so idle work starts immediately and survives process
recovery. Child prompt runs open their steering inbox before execution is
announced, allowing messages received during startup to join the active turn.

Each open child also owns a watch-backed status stream using Codex's lifecycle
states: `pending_init`, `running`, `interrupted`, `completed`, `errored`,
`shutdown`, and `not_found`. Current `wait_agent` uses a lossless parent activity
queue, returns on any child result, and wakes early when the user steers the
active turn. Its default timeout is 30 seconds with Codex's 10-second to
one-hour range. The hidden target-list compatibility path still subscribes to
status streams; `interrupted` remains reusable and intentionally non-final.

Interrupting an active turn waits for its runtime to settle, then records a
durable developer-role marker explaining that tools may have partially run.
This marker is included in later prompts and interrupted `fork_turns`
snapshots. OpenAI chat and Responses requests preserve the native `developer`
role; providers without that role fold the marker into system instructions.
The marker is enabled by default, matching Codex. Disable it globally with:

```toml
[agents]
interrupt_message = false
```

Closing is also mailbox-safe and subtree-aware. The selected child is marked
closed, then every resident descendant is shut down while its durable spawn
edge remains available for later lazy reload. If any affected child was
processing a queued follow-up, that receipt returns to pending before the
thread is removed. Repeating a close is safe. In the V2 lifecycle, resume
reopens only the selected child; a later message to a descendant reloads that
same descendant identity and dispatches its preserved FIFO queue.

Messaging a known closed child transparently reloads it with the same durable
ID, transcript, and mailbox. Durable spawns and reloads share an atomic pending
slot ledger, so concurrent setup counts toward the six-slot limit before a
child is registered. Successful setup commits the slot into LRU residency;
failure or cancellation releases it automatically. At capacity, either path
unloads the least recently used safe child first. Only idle children in a
completed, errored, or interrupted state with an empty mailbox are eligible;
active children and pending work remain protected. Explicit `resume_agent` uses
this same residency path and remains available for lifecycle control. Reload
coordination is keyed by canonical durable child ID: name and ID aliases join
one activation, while unrelated children can reload concurrently. Weak lock
entries are pruned after the activation waiters release them.
An actual reload overlays the parent's live model selector, workspace, and
prior-context ceiling before queued work can run. The workspace-bearing child
system instruction is regenerated and the refreshed session is saved. Children
restored directly from disk receive that overlay once on first access; already
live children retain their active configuration, matching Codex's fast path.

The legacy `agent` tool remains available for compatibility. Name lookup is
scoped to the owning parent, so different parents may use the same nickname;
an unscoped ambiguous nickname is rejected instead of being misrouted. Spawn
setup atomically reserves `(owner, nickname)` before asynchronous construction,
so concurrent same-parent requests cannot create ambiguous children. Failed or
cancelled setup releases the claim for retry, while separate parents may still
use the same nickname. Spawned entries now derive `parent` and `depth` from the
owning child session, so nested agents retain real hierarchy metadata.
`fork_turns` accepts `none`, `all`, or a positive count and inherits the
requested complete parent turns without copying the in-flight tool-call tail.
Children also inherit the workspace and parent context-access policy.
Nesting defaults to one child level, matching Codex; set
`agents.max_depth` to an unsigned integer to opt into deeper trees. Open child
sessions default to the Codex limit of six and use the canonical
`agents.max_concurrent_threads_per_session` setting. The Codex-compatible
`agents.max_threads` alias is also accepted:

```toml
[agents]
max_threads = 6
max_depth = 1
```

The legacy `CODETETHER_AGENT_MAX_THREADS` and `CODETETHER_AGENT_MAX_DEPTH`
environment overrides remain supported and take precedence over TOML. Close an
agent for later reuse, or permanently remove its registration and mailbox with
the legacy `kill` action.
