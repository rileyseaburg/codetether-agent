//! Native session operations used by resumable runs.
use anyhow::Result;
use codetether_provider_core::Message;
use codetether_run_checkpoint::storage::Storage;
use codetether_session_events::{SessionEvent, SessionResult};
use std::sync::Arc;
use tokio::sync::mpsc;
/// Session execution and fields required by the resume loop.
/// # Implementors
/// Applications implement this on their durable session alongside [Storage].
#[async_trait::async_trait(?Send)]
pub trait RunSession: Storage + Sized {
    /// Native registry shared across streaming attempts.
    type Registry;
    /// Current transcript used to detect progress and build checkpoints.
    fn messages(&self) -> &[Message];
    /// Update the step limit for the next attempt.
    fn set_max_steps(&mut self, max_steps: Option<usize>);
    /// Application default used only when no limit was supplied.
    fn default_max_steps() -> usize;
    /// Load the native registry once before streaming begins.
    /// # Errors
    /// Returns provider initialization errors.
    async fn load_registry() -> Result<Arc<Self::Registry>>;
    /// Execute a plain prompt attempt.
    /// # Errors
    /// Returns native execution errors.
    async fn prompt(&mut self, message: &str) -> Result<SessionResult>;
    /// Execute an attempt with live events and the existing registry.
    /// # Errors
    /// Returns native execution errors.
    async fn prompt_with_events(
        &mut self,
        message: &str,
        events: mpsc::Sender<SessionEvent<Self>>,
        registry: Arc<Self::Registry>,
    ) -> Result<SessionResult>;
}
