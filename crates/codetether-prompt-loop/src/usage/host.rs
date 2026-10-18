use crate::LoopState;
use codetether_session_events::SessionEvent;
use std::{future::Future, time::Duration};
use tokio::sync::mpsc;
/// Native model identity, goal accounting and live event services.
/// # Implementors
/// The prompt runner retains its model, goal store and accounting baseline.
pub trait Host: Send {
    /// Native snapshot carried by session events.
    type Snapshot: Send;
    /// Current provider and model identifiers, in that order.
    fn target(&self) -> (&str, &str);
    /// Mutable elapsed-time baseline for durable goal accounting.
    fn progress(&mut self) -> &mut LoopState;
    /// Record consumed tokens and elapsed time in the goal store.
    /// # Errors
    /// Returns storage failures for best-effort logging by the caller.
    fn account(
        &mut self,
        tokens: usize,
        elapsed: Duration,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Optional live usage-report sender.
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent<Self::Snapshot>>>;
}
