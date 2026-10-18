use codetether_chat_model::{ChatMessage, MessageType};
use std::future::Future;
/// Native UI operations needed to report missing provider configuration.
/// # Implementors
/// UI adapters retain worker-bridge, timing, status and scroll effects.
pub trait FailureHost {
    /// Stop processing through the current worker bridge.
    fn processing_stopped(&mut self) -> impl Future<Output = ()>;
    /// Clear active request timing.
    fn clear_timing(&mut self);
    /// Append a visible error message.
    fn push(&mut self, message: ChatMessage);
    /// Replace the visible status text.
    fn status(&mut self, message: String);
    /// Scroll to the latest chat entry.
    fn scroll(&mut self);
}
/// Report missing providers after resetting native request state.
/// # Arguments
/// Host supplies the current UI and worker-bridge effects.
/// # Returns
/// Completes after publishing the error and final status.
/// # Examples
/// ```ignore
/// no_provider(&mut ui).await;
/// ```
pub async fn no_provider<H: FailureHost>(host: &mut H) {
    host.processing_stopped().await;
    host.clear_timing();
    host.push(ChatMessage::new(
        MessageType::Error,
        "No providers available. Configure credentials first.",
    ));
    host.status("No providers configured".to_string());
    host.scroll();
}
