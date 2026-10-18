//! Native registry, policy and context services for the shared agent runtime.
//!
//! [`Host`] combines an application tool catalog with [`SessionServices`].
//! Native code owns registry lookup, authorization, goal/peer composition and
//! context completion. Applications retain session access and input enrichment.
use codetether_agent_runtime::SessionWrite;
use serde_json::Value;
use std::{marker::PhantomData, path::Path};
mod context;
mod registry;
mod session;

/// Application session mutations and enrichment used by native agent services.
/// # Implementors
/// Implement the shared session view/write traits and preserve the existing
/// session-aware enrichment call. The session also supplies native context requests.
pub trait SessionServices: SessionWrite {
    /// Enrich tool input for a workspace using the session's current state.
    fn enrich(input: &Value, cwd: &Path, session: &Self::Session) -> Value;
}

/// Shared-agent host using session services `S` and registry application `T`.
/// # Examples
/// The application selects this type as the host of `codetether_agent_runtime::Agent`.
/// ```no_run
/// # fn example<S, T>() {
/// let _: Option<codetether_native_agent::Host<S, T>> = None;
/// # }
/// ```
pub struct Host<S, T>(PhantomData<fn() -> (S, T)>);

mod ephemeral;
/// Shared provider registry cache for native agent-tool actions.
pub mod provider_cache;
pub use ephemeral::Ephemeral;

mod mailbox;
pub use mailbox::{Execution as MailboxExecution, Mailbox};
