//! Registry and authorization services supplied by the application.
use async_trait::async_trait;
use codetether_tool_core::Tool;
use codetether_tool_reporting::ToolTuple;
use serde_json::Value;
use std::sync::Arc;
/// Supplies tool lookup and the ordered policy and workspace gates.
///
/// # Implementors
/// Applications resolve their own registry and preserve their authorization services.
#[async_trait]
pub trait Host: Sync {
    /// Resolve a registered tool; missing tools are reported before either gate.
    fn resolve(&self, name: &str) -> Option<Arc<dyn Tool>>;
    /// Return a blocking policy result, or None to proceed to coordination.
    async fn policy(&self, name: &str, args: &Value) -> Option<ToolTuple>;
    /// Return a blocking workspace-coordination result, or None to execute.
    async fn coordinate(&self, name: &str, args: &Value) -> Option<ToolTuple>;
}
