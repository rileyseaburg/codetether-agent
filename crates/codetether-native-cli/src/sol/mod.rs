//! Native Sol planning, coding turns and touched-file diagnostics.
//!
//! [Host] binds the application's [Session] and validation adapter. [Worker]
//! retains the coding session and resume limits between implementation turns.
use codetether_agent_core::ToolUse;
use codetether_config::Config;
mod host;
mod worker;
pub use host::Host;
pub use worker::Worker;
/// Session policy and fields required by the native Sol coding worker.
/// # Implementors
/// Use the application's ordinary run/resume and session-configuration methods.
pub trait Session:
    codetether_run_resume::SessionSource
    + codetether_run_resume::RunSession
    + codetether_run_policy::SessionPolicy
{
    /// Set the agent identity used in session events and metadata.
    fn set_agent_name(&mut self, name: &str);
    /// Apply the normal configuration with no separate model override.
    fn apply_config(&mut self, config: &Config);
    /// Current model or an empty string when unconfigured.
    fn model(&self) -> String;
    /// Ordered tool-use records used for touched-file diagnostics.
    fn tool_uses(&self) -> &[ToolUse];
}
