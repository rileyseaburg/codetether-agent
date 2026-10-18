//! Borrowed execution inputs for tool-output processing.
use codetether_provider_core::Provider;
use std::{path::Path, sync::Arc, time::Instant};

/// Provider, model, deadline, and working directory for a swarm agent.
///
/// # Examples
/// ```no_run
/// # fn example(provider: &std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let _context = codetether_swarm_output::Context {
///     provider, model: "model", deadline: std::time::Instant::now(), working_dir: None,
/// };
/// # }
/// ```
pub struct Context<'a> {
    /// Shared provider handle, cloned only when summarization is needed.
    pub provider: &'a Arc<dyn Provider>,
    /// Model identifier for the RLM executor.
    pub model: &'a str,
    /// Deadline shared with the owning agent loop.
    pub deadline: Instant,
    /// Working directory included in tool feedback.
    pub working_dir: Option<&'a Path>,
}
