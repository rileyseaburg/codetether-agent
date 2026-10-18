//! Controlled worker swarm execution and standard-session continuation.
//!
//! [`execute`] preserves preparation, execution control, result persistence and
//! handoff order. [`Session`] supplies native title generation and prior-context
//! permission; other session services use existing shared contracts.
use codetether_a2a_worker_core::session_protocol::policy;
use codetether_application_registry::{Application, ToolRegistry};
use codetether_native_providers::ProviderRegistry;
use codetether_session_events::SessionResult;
use std::future::Future;
mod context;
mod execute;
mod finish;
mod handoff;
mod prepare;
pub use execute::execute;

/// Application session services used during swarm preparation and handoff.
/// # Implementors
/// Native sessions retain their permission and title behavior while implementing
/// the shared metadata, transcript, context and worker policy contracts.
pub trait Session:
    crate::session_loop::Session
    + crate::tool_call::InputContext
    + crate::preparation::SessionState
    + policy::Source<Registry = ProviderRegistry>
{
    /// Whether prior-context access permits unrestricted swarm execution.
    fn swarm_prior_context_allowed(&self) -> bool;
    /// Generate the missing title using the application's usual title path.
    /// # Errors
    /// Propagates title-generation errors.
    fn generate_swarm_title(&mut self) -> impl Future<Output = anyhow::Result<()>> + Send;
}

/// Reject unrestricted swarm execution when prior-context access is denied.
/// # Errors
/// Returns the existing source-restriction error when access is not allowed.
pub fn ensure_allowed(session: &impl Session) -> anyhow::Result<()> {
    if session.swarm_prior_context_allowed() {
        return Ok(());
    }
    anyhow::bail!(
        "Swarm execution is disabled while prior memory/session/history access is denied; use session execution so the user's source restriction remains enforced"
    )
}
