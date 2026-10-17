//! Application operations consumed by the shared worker loop.
use anyhow::Result;
use async_trait::async_trait;

/// Reason to finish or continue after a model message.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::session_protocol::driver::Continuation;
/// let outcome = Continuation::Goal;
/// assert!(matches!(outcome, Continuation::Goal));
/// ```
pub enum Continuation {
    /// No pending steering or goal work remains.
    Finish,
    /// Continue the existing goal, clearing prior output accumulation.
    Goal,
    /// Continue after accepted steering, retaining accumulated output.
    Steering,
}

/// Application operations required to run a worker's bounded session loop.
///
/// # Implementors
/// Implementations own session mutation, control-plane checks, provider/tool
/// execution, and persistence. Methods are called sequentially by [super::run].
#[async_trait]
pub trait SessionLoopHost: Send {
    /// Check cancellation, drain steering, and apply the wrap-up boundary.
    /// # Errors
    /// Returns any control-boundary error before a provider step begins.
    async fn before_step(&mut self, step: usize, max_steps: usize) -> Result<()>;
    /// Execute and record a provider/tool step, appending output in order.
    /// # Errors
    /// Returns provider, tool-processing, or session-control errors.
    async fn process_step(&mut self, output: &mut String) -> Result<bool>;
    /// Decide what should happen after a response containing no tool calls.
    /// # Errors
    /// Returns continuation journal or session-control errors.
    async fn continuation(&mut self) -> Result<Continuation>;
    /// Record and persist a visible diagnostic for a budget-exhausted loop.
    async fn record_halt(&mut self, reason: &str);
    /// Persist final session state.
    /// # Errors
    /// Returns any final persistence error.
    async fn save(&mut self) -> Result<()>;
}
