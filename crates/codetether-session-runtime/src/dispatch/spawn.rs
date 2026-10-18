//! Command-channel creation and owned dispatcher task startup.
use super::Host;
use crate::handle::{Control, SessionHandle};
use tokio::sync::mpsc;

/// Start a dispatcher with a shared direct cancellation controller.
/// # Arguments
/// * `host` — Builds the owned native adapter with a clone of the controller
///   retained by the returned handle.
/// # Returns
/// A handle for the eight-command buffered channel and direct control signals.
/// # Panics
/// Panics if called outside an active Tokio runtime.
/// # Examples
/// ```no_run
/// # fn example<Q: Send + 'static, C: codetether_session_runtime::handle::Control + Default + Clone, H: codetether_session_runtime::dispatch::Host<Q> + 'static>(host: impl FnOnce(C) -> H) {
/// let handle = codetether_session_runtime::dispatch::spawn::<Q, C, H>(host);
/// drop(handle);
/// # }
/// ```
pub fn spawn<Q, C, H>(host: impl FnOnce(C) -> H) -> SessionHandle<Q, C>
where
    Q: Send + 'static,
    C: Control + Default + Clone,
    H: Host<Q> + 'static,
{
    let (cmd_tx, cmd_rx) = mpsc::channel(8);
    let active_cancel = C::default();
    let mut host = host(active_cancel.clone());
    tokio::spawn(async move {
        super::run(&mut host, cmd_rx).await;
    });
    SessionHandle::new(cmd_tx, active_cancel)
}
