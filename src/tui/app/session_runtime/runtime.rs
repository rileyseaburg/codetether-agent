//! Native event and notice channel binding for shared runtime startup.
use super::{SessionNotice, TuiSessionHandle};
use crate::session::SessionEvent;
use tokio::sync::mpsc;

/// Spawn the TUI-owned session runtime task.
///
/// The returned [`TuiSessionHandle`] sends commands to the spawned task. The
/// task continues until it receives a shutdown command or its command channel is
/// closed.
///
/// # Arguments
///
/// * `event_tx` - Channel used by prompt execution to stream [`SessionEvent`]
///   values back to the TUI.
/// * `notice_tx` - Channel used by the runtime to report [`SessionNotice`]
///   lifecycle updates such as prompt start, completion, or failure.
///
/// # Returns
///
/// A handle connected to the runtime command channel.
///
/// # Side Effects
///
/// Spawns a Tokio task and allocates an internal command channel with capacity
/// for eight queued commands.
pub(crate) fn spawn(
    event_tx: mpsc::Sender<SessionEvent>,
    notice_tx: mpsc::Sender<SessionNotice>,
) -> TuiSessionHandle {
    codetether_session_runtime::dispatch::spawn(|cancel| super::dispatch_host::Native {
        cancel,
        event_tx,
        notice_tx,
    })
}
