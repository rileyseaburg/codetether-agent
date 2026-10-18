use crate::execution_policy::AutoApprove;
use std::sync::Arc;

/// Constant per-run inputs needed to process a step response.
/// # Examples
/// ```
/// use codetether_a2a_worker_core::{execution_policy::AutoApprove, session_protocol::context::ResponseContext};
/// let context = ResponseContext { model: "model", tool_registry: &(), auto_approve: AutoApprove::Safe,
///     workspace_dir: std::path::Path::new("."), output_callback: None };
/// assert_eq!(context.model, "model");
/// ```
pub struct ResponseContext<'a, R> {
    /// Model used to attribute response usage and tool execution.
    pub model: &'a str,
    /// Executable tool registry supplied by the host.
    pub tool_registry: &'a R,
    /// Worker approval policy, passed through unchanged.
    pub auto_approve: AutoApprove,
    /// Effective working directory for tools.
    pub workspace_dir: &'a std::path::Path,
    /// Optional text output observer.
    pub output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
}
