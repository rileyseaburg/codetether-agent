//! Frontend state and prompt-dispatch boundaries.
use codetether_a2a_inbox::Inbound;
use codetether_worker_events::IncomingTask;
/// Chat presentation for A2A command and task notices.
/// # Implementors
/// Frontends append system text to their existing transcript.
pub trait Presentation {
    /// Append a system notice.
    fn push_system(&mut self, text: String);
    /// Scroll to the notice.
    fn scroll_to_bottom(&mut self);
    /// Replace the status message.
    fn set_status(&mut self, status: String);
}
/// State used when admitting a parked peer turn.
/// # Implementors
/// Frontends retain the in-flight responder and incoming task queue.
pub trait Inbox {
    /// Whether the frontend is currently processing a turn.
    fn processing(&self) -> bool;
    /// Current parked turn owned by the frontend.
    fn inbound(&mut self) -> &mut Option<Inbound>;
    /// Queue the admitted turn as ordinary remote work.
    fn enqueue_task(&mut self, task: IncomingTask);
}
/// Session/provider operations needed to dispatch remote work.
/// # Implementors
/// Frontends use their normal prompt path with no image attachments.
#[allow(async_fn_in_trait)]
pub trait Dispatch: Presentation {
    /// Whether a turn is currently running.
    fn processing(&self) -> bool;
    /// Take the oldest pending task.
    fn dequeue_task(&mut self) -> Option<IncomingTask>;
    /// Track the task whose completion will be reported.
    fn set_active_task(&mut self, task: IncomingTask);
    /// Dispatch using the frontend's active session and provider registry.
    async fn dispatch_prompt(&mut self, prompt: &str);
}
