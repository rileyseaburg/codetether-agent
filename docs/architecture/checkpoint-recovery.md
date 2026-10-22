# Checkpoint recovery over canonical history

The CLI resume loops compare absolute `message_count()` values. A full working
window can therefore keep its size while new durable messages still trigger an
exhaustion checkpoint.

The shared session `build_run_checkpoint` method saves pending messages, then
folds canonical records in a single SQLite read transaction. It uses the
session's actual storage path, including custom locations. The fold validates
contiguous sequence numbers and the snapshot header's message count, resolves
blob references, and releases each raw message after extraction. Missing or
corrupt records fail checkpoint creation rather than producing partial evidence.

`CheckpointBuilder` keeps source-linked user text, tool attempts and results,
the latest assistant observation, inferred next action, and unresolved failures.
A call and result may be separated by working-window eviction; scanning the
canonical sequence still matches them. Ordinary checkpoints retain all extracted
source text in JSON. Capture makes one full sequential pass on exhaustion;
it does not copy the whole raw transcript into memory. This is deliberately a
snapshot rebuild so historical edits are reflected without trusting stale
incremental summaries.

Resume validates the checkpoint's session identity and reconciles its original
objective and budget with fresh canonical evidence, including same-length
corrections. The original objective remains stored in full. Model-facing text
uses bounded UTF-8-safe excerpts, with both the beginning and end of long text.
The preview preserves the original user request and recent corrections, labels
omissions, and directs source recovery through `context_browse` before dependent
actions. It identifies the complete user-source span. The complete extracted
evidence remains in the checkpoint and original messages remain in the database.

Checkpoint saves commit canonical metadata first, then write a synchronized
temporary sidecar and atomically replace the previous sidecar. A failed metadata
save restores the previous in-memory checkpoint and leaves the sidecar unchanged.
If publishing the sidecar fails, the error reports that canonical metadata has
already committed; the valid embedded checkpoint remains recoverable.

This does not make assistant plans verified outcomes. Successful tool execution
is still distinct from completing the user's task. The separate checkpoint-clear
path commits a durable `run_checkpoint_cleared` marker before removing the old
sidecar. Loading honors that marker even when a stale or malformed sidecar
survives interruption. A failed canonical clear restores the previous in-memory
checkpoint and flag; a failed sidecar cleanup leaves the committed marker intact.
New checkpoint saves reset the marker, with rollback if the canonical save fails.
