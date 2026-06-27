//! Per-frame handling for the resumable worker task stream.
//!
//! Classifies a parsed SSE frame and routes it: advisory frames dispatch a task
//! handler, control frames are observed, sequenced frames dispatch then commit
//! the resume cursor. See `docs/transport-phase1-wire-contract.md`.

use crate::a2a::stream::classify::{EventClass, classify};
use crate::a2a::stream::cursor::Cursor;
use crate::a2a::stream::frame::ParsedFrame;
use crate::a2a::stream::resync::parse_resync;

use super::{
    WorkerTaskRuntime, pending_tasks::poll_pending_tasks, task_dispatch::spawn_task_handler,
};

/// Handle one parsed frame against the runtime, advancing `cursor` for
/// sequenced events only after their side effects are dispatched.
pub(super) async fn handle_frame(
    frame: &ParsedFrame,
    runtime: &WorkerTaskRuntime,
    cursor: &mut Cursor,
) {
    if frame.event == "resync-required" {
        return handle_resync(frame, runtime, cursor).await;
    }
    if frame.data == "[DONE]" || frame.data.is_empty() {
        return;
    }
    let Ok(task) = serde_json::from_str::<serde_json::Value>(&frame.data) else {
        return;
    };
    match classify(&frame.event) {
        EventClass::Control => {}
        EventClass::Advisory => spawn_task_handler(&task, runtime).await,
        EventClass::Sequenced => {
            spawn_task_handler(&task, runtime).await;
            if let Some(id) = &frame.id
                && let Err(error) = cursor.commit(id)
            {
                tracing::warn!(error = %error, "Failed to commit stream cursor");
            }
        }
    }
}

/// Drop the resume cursor and cold-reconcile pending tasks on `resync-required`.
async fn handle_resync(frame: &ParsedFrame, runtime: &WorkerTaskRuntime, cursor: &mut Cursor) {
    match parse_resync(&frame.data) {
        Ok(resync) => tracing::warn!(reason = ?resync.reason, epoch = %resync.epoch, "Stream resync required"),
        Err(error) => tracing::warn!(error = %error, "Malformed resync-required payload"),
    }
    if let Err(error) = cursor.reset() {
        tracing::warn!(error = %error, "Failed to reset stream cursor on resync");
    }
    if let Err(error) = poll_pending_tasks(runtime).await {
        tracing::warn!(error = %error, "Cold reconcile after resync failed");
    }
}
