use codetether_chat_model::ChatMessage;
/// Native UI and watchdog mutations for continuation.
/// # Implementors
/// UI adapters retain concrete watchdog notifications and request state.
pub trait Ui {
    /// Clear the submitted command input.
    fn clear_input(&mut self);
    /// Reset the watchdog restart budget.
    fn set_restarts(&mut self, count: u32);
    /// Retain the prompt to resubmit after cancellation.
    fn set_inflight(&mut self, prompt: String);
    /// Construct and install a watchdog retry notification.
    fn arm_retry(&mut self, message: String, count: u32);
    /// Set the native processing flag.
    fn set_processing(&mut self, processing: bool);
    /// Clear streaming display state.
    fn clear_streaming(&mut self);
    /// Clear active request timing.
    fn clear_timing(&mut self);
    /// Set the visible status text.
    fn status(&mut self, message: String);
    /// Borrow the chat log for continuation feedback.
    fn messages_mut(&mut self) -> &mut Vec<ChatMessage>;
    /// Publish resumed text as a user message.
    fn publish_user(&mut self, prompt: &str);
    /// Scroll to the newest message.
    fn scroll(&mut self);
}
