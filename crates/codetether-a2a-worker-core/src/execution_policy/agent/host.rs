use crate::session_protocol::policy::PolicySessionResult;
use codetether_session_events::SessionResult;
use std::future::Future;
/// Native session and swarm execution for one configured worker task.
/// # Implementors
/// Adapters retain the task context, approval policy, bus and output callback.
pub trait Host: Send {
    /// Execute the task through swarm orchestration.
    /// # Errors
    /// Propagates native swarm failures.
    fn swarm(&mut self) -> impl Future<Output = anyhow::Result<(SessionResult, bool)>> + Send;
    /// Execute the task as a standard worker session.
    /// # Errors
    /// Propagates native session failures.
    fn standard(&mut self) -> impl Future<Output = anyhow::Result<PolicySessionResult>> + Send;
}
