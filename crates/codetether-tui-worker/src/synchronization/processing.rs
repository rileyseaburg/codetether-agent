use crate::{WorkerBridgeCmd, WorkerState};
use tokio::sync::mpsc;

/// Record a processing transition and attempt to report it to the bridge.
///
/// # Arguments
/// * `state` - Worker state storing the last reported processing value.
/// * `processing` - Main UI processing flag to update first.
/// * `sender` - Optional bridge command sender.
/// * `value` - New processing value.
/// # Returns
/// Updates both local flags even if the command cannot be sent.
/// # Examples
/// ```
/// use codetether_tui_worker::{synchronization, WorkerState};
/// let mut state = WorkerState::default();
/// let mut processing = false;
/// synchronization::set_processing(&mut state, &mut processing, None, true);
/// assert!(processing);
/// assert_eq!(state.worker_bridge_processing_state, Some(true));
/// ```
pub fn set_processing(
    state: &mut WorkerState,
    processing: &mut bool,
    sender: Option<&mpsc::Sender<WorkerBridgeCmd>>,
    value: bool,
) {
    *processing = value;
    state.sync_worker_bridge_processing(value);
    let _ = super::send(sender, WorkerBridgeCmd::SetProcessing(value));
}
