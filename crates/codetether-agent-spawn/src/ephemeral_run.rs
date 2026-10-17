//! Prepared native resources consumed by an ephemeral execution.
use codetether_provider_core::Provider;
use std::{path::PathBuf, sync::Arc};
/// Provider, prompt, executable tools and workspace for one child invocation.
/// # Examples
/// Hosts receive this value through [crate::EphemeralHost::execute]. The fields
/// replace the corresponding native loop arguments without changing ownership.
pub struct EphemeralRun<R> {
    /// Resolved provider handle.
    pub provider: Arc<dyn Provider>,
    /// Provider-specific resolved model ID.
    pub model: String,
    /// Existing direct-worker prompt with isolation guidance appended.
    pub prompt: String,
    /// Native executable tool registry.
    pub registry: Arc<R>,
    /// Final working directory, isolated when the task permits mutation.
    pub workspace: PathBuf,
}
