use super::Attachments;
use std::future::Future;
/// Goal adoption, visible chat publication and final provider dispatch.
/// # Implementors
/// UI adapters bind existing native session and provider operations.
pub trait Actions: Attachments {
    /// Attempt to adopt the expanded prompt as a goal.
    fn adopt(&mut self, agent_prompt: &str) -> impl Future<Output = ()>;
    /// Publish expanded prompt text and pending images to the chat log.
    fn publish(&mut self, prompt: &str, images: &[Self::Image]);
    /// Dispatch expanded prompt text and the owned pending images.
    fn dispatch(
        &mut self,
        agent_prompt: &str,
        images: Vec<Self::Image>,
    ) -> impl Future<Output = ()>;
}
