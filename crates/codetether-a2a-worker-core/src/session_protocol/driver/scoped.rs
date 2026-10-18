use super::SessionLoopHost;
use crate::session_protocol::SessionStepsOutcome;
use codetether_session_control::{ExecutionScope, GateFuture};
/// Native session identity and durable execution gate for a worker loop.
/// # Implementors
/// Extend the existing worker loop host with its session ID and native gate.
pub trait ScopedHost: SessionLoopHost {
    /// Durable session identifier used for exclusive execution ownership.
    fn session_id(&self) -> &str;
    /// Check native cancellation and durable goal holds.
    fn gate(session_id: &str) -> GateFuture<'_>;
}
/// Run a worker loop under an exclusive or explicitly inherited session scope.
/// # Arguments
/// `host` supplies loop effects and the durable gate; `max_steps` is clamped
/// to at least one step before running the loop.
/// # Returns
/// The bounded worker-loop outcome after scope execution.
/// # Errors
/// Propagates initial gate and scope acquisition errors directly. Loop errors
/// are recorded as visible halt diagnostics before being returned.
/// # Examples
/// ```no_run
/// # async fn run(host: &mut impl codetether_a2a_worker_core::session_protocol::driver::ScopedHost) -> anyhow::Result<()> {
/// let _outcome = codetether_a2a_worker_core::session_protocol::driver::run_scoped(host, 50).await?;
/// # Ok(()) }
/// ```
pub async fn run_scoped<H: ScopedHost>(
    host: &mut H,
    max_steps: usize,
) -> anyhow::Result<SessionStepsOutcome> {
    let session_id = host.session_id().to_owned();
    H::gate(&session_id).await?;
    let mut control = ExecutionScope::open(&session_id, H::gate)?;
    let result = control.run(super::run(host, max_steps.max(1))).await;
    if let Err(error) = &result {
        host.record_halt(&error.to_string()).await;
    }
    result
}
