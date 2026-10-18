use super::Control;
use crate::request::SessionCommand;
use tokio::sync::mpsc;
/// Cloneable command sender with a native active-turn control handle.
/// # Examples
/// ```no_run
/// # fn make<Q, C: codetether_session_runtime::handle::Control + Clone>(control: C) {
/// let (tx, _rx) = tokio::sync::mpsc::channel::<codetether_session_runtime::request::SessionCommand<Q>>(8);
/// let handle = codetether_session_runtime::handle::SessionHandle::new(tx, control);
/// let _other = handle.clone();
/// # }
/// ```
pub struct SessionHandle<Q, C: Control> {
    pub(super) tx: mpsc::Sender<SessionCommand<Q>>,
    pub(super) active_cancel: C,
}
impl<Q, C: Control + Clone> Clone for SessionHandle<Q, C> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
            active_cancel: self.active_cancel.clone(),
        }
    }
}
impl<Q, C: Control> SessionHandle<Q, C> {
    /// Wrap the command channel and direct active-turn controller.
    /// # Arguments
    /// `tx` accepts commands; `active_cancel` retains native active-turn policy.
    /// # Returns
    /// A handle that transfers requests and sends control messages.
    /// # Examples
    /// ```no_run
    /// # fn make<Q, C: codetether_session_runtime::handle::Control>(control: C) {
    /// let (tx, _rx) = tokio::sync::mpsc::channel(8);
    /// let _handle: codetether_session_runtime::handle::SessionHandle<Q, C> =
    ///     codetether_session_runtime::handle::SessionHandle::new(tx, control);
    /// # }
    /// ```
    pub fn new(tx: mpsc::Sender<SessionCommand<Q>>, active_cancel: C) -> Self {
        Self { tx, active_cancel }
    }
}
