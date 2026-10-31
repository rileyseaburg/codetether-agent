//! Acknowledge local state only after the queued write has committed.
use crate::{Checkpoint, Snapshot, pin_edits::PinSource};

pub(crate) fn apply<S: Snapshot>(
    session: &S,
    mut checkpoint: Checkpoint,
    edits: Vec<PinSource>,
    revision: (i64, (usize, usize)),
    header: String,
) {
    checkpoint.revision = revision.0;
    checkpoint.message_end = revision.1.0;
    checkpoint.tool_end = revision.1.1;
    checkpoint.header = header;
    for edit in edits {
        checkpoint
            .constraints
            .retain(|source| source.source_turn != edit.index);
        if let Some(source) =
            crate::constraints::ConstraintRecord::from_message(edit.index, &edit.message, edit.page)
        {
            checkpoint.constraints.push(source);
        }
    }
    checkpoint
        .constraints
        .sort_by_key(|source| source.source_turn);
    checkpoint.pin_sources.clear();
    *session.state().0.lock().unwrap() = checkpoint;
    crate::checkpoint::clean(session);
}
