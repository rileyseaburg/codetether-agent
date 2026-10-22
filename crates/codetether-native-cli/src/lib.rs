//! Native command-line parsing and standalone command services.
//!
//! [Cli] and [Command] retain application help/version metadata. Standalone
//! auth, Vault, browser, model and search handlers use their owning service
//! crates. Session-running commands are bound by the embedding application.
mod cli;
pub use cli::*;

/// Native Sol planner and stateful coding-worker integration.
pub mod sol;

mod run_binding;
/// Types used by the application-local CLI session binding.
#[doc(hidden)]
pub mod __private {
    pub use anyhow::Result;
    pub use async_trait::async_trait;
    pub use codetether_bus_core::AgentBus;
    pub use codetether_cli_relay::Session as RelaySession;
    pub use codetether_native_providers::ProviderRegistry;
    pub use codetether_provider_core::{ContentPart, Message, Role, Usage};
    pub use codetether_run_policy::SessionPolicy;
    pub use codetether_run_resume::{RunCheckpoint, RunSession, SessionSource};
    pub use codetether_run_runtime::Session as RuntimeSession;
    pub use codetether_session_events::{SessionEvent, SessionResult};
    pub use tokio::sync::mpsc;
}
