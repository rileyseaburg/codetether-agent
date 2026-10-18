//! Owned prompt task and events supplied by the application.
use codetether_session_events::{SessionEvent, SessionResult};
use tokio::{sync::mpsc, task::JoinHandle};
/// Terminal join result from the application prompt task.
pub type PromptOutcome = Result<Result<SessionResult, String>, tokio::task::JoinError>;
/// Application callback that starts one task with its event channel.
pub type PromptStarter<S> = fn(String, String, Option<String>) -> PromptRun<S>;
/// One prompt's event receiver and task; cancellation waits for resource cleanup.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let (_tx, events) = tokio::sync::mpsc::channel::<codetether_session_events::SessionEvent<()>>(1);
/// let task = tokio::spawn(async { std::future::pending().await });
/// let mut run = codetether_session_realtime::PromptRun { events, task };
/// run.cancel().await;
/// # }
/// ```
pub struct PromptRun<S> {
    /// Events emitted by the owned prompt task.
    pub events: mpsc::Receiver<SessionEvent<S>>,
    /// Terminal result from the owned task.
    pub task: JoinHandle<Result<SessionResult, String>>,
}
impl<S> PromptRun<S> {
    /// Aborts the prompt and waits until its owned resources are dropped.
    pub async fn cancel(&mut self) {
        self.task.abort();
        let _ = (&mut self.task).await;
    }
}
#[cfg(test)]
#[path = "prompt_run_tests.rs"]
mod tests;
