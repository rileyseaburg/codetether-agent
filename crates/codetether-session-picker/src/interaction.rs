//! Load-then-resume sequencing for selected sessions.
use crate::SessionPickerState;
use std::future::Future;
/// Native session ownership and continuation operations.
/// # Implementors
/// Frontends expose the picker and retain storage/runtime transfer operations.
pub trait Host {
    /// Borrow current picker state.
    fn picker(&self) -> &SessionPickerState;
    /// Load the requested session if the native slot is available.
    fn load_selected(&mut self, session_id: &str) -> impl Future<Output = ()>;
    /// Borrow the available session's identity after loading.
    fn current_session_id(&self) -> Option<&str>;
    /// Attempt the existing automatic goal continuation.
    fn resume(&mut self) -> impl Future<Output = bool>;
}
/// Load the selected session and resume only when that identity is current.
/// # Arguments
/// Receives the picker and native session operations.
/// # Returns
/// Leaves absent or unsuccessfully loaded selections without continuation.
/// # Examples
/// ```no_run
/// async fn select<H: codetether_session_picker::interaction::Host>(host: &mut H) {
///     codetether_session_picker::interaction::select(host).await;
/// }
/// ```
pub async fn select(host: &mut impl Host) {
    let session_id = host.picker().selected_session_id();
    if let Some(session_id) = session_id {
        host.load_selected(&session_id).await;
        if host
            .current_session_id()
            .is_some_and(|current| current == session_id)
        {
            host.resume().await;
        }
    }
}
