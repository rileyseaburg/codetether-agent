use super::Target;
use codetether_provider_core::Message;
use std::future::Future;
/// Native history and preparation services for a completion attempt.
///
/// # Implementors
/// Implementations retain ownership of the session, model and derived history.
pub trait Host: Send {
    /// Read the current model settings.
    fn target(&self) -> Target<'_>;
    /// Read optional proactive context collected for this attempt.
    fn proactive(&self) -> Option<&Message>;
    /// Prepare derived history and append recalled context to extras.
    fn prepare(&mut self, extras: &mut Vec<Message>) -> impl Future<Output = ()> + Send;
    /// Read derived history after preparation.
    fn history(&self) -> &[Message];
}
