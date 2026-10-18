use crate::WorkerBridgeCmd;
use tokio::sync::mpsc;

/// Try to send one bridge command without waiting for channel capacity.
///
/// # Arguments
/// * `sender` - Command sender when a bridge exists.
/// * `command` - Command transferred to the channel on success.
/// # Returns
/// Whether the command was accepted; absent, closed or full channels return false.
/// # Examples
/// ```
/// use codetether_tui_worker::{synchronization::send, WorkerBridgeCmd};
/// assert!(!send(None, WorkerBridgeCmd::Shutdown));
/// ```
pub fn send(sender: Option<&mpsc::Sender<WorkerBridgeCmd>>, command: WorkerBridgeCmd) -> bool {
    match sender {
        Some(sender) => sender.try_send(command).is_ok(),
        None => false,
    }
}
