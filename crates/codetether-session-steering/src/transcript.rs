//! Native transcript operations required when consuming queued inputs.
use codetether_provider_core::Message;
/// Existing session transcript and user-prompt publication services.
/// # Implementors
/// Applications implement this trait on their session type. Input insertion
/// must retain the same user-message bookkeeping as ordinary human prompts.
pub trait Transcript {
    /// Durable session identity used to select the input inbox.
    fn id(&self) -> &str;
    /// Append a trusted human message with the native session bookkeeping.
    fn add_human_message(&mut self, message: Message);
    /// Publish user text using the session's existing event/bus context.
    fn publish_user_prompt(&self, text: &str);
}
