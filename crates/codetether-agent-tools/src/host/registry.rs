//! Tool registry and invocation-policy services.
use crate::*;
/// Access the same native tool instances and policy evaluator.
/// # Implementors
/// Native agents return their existing registry entries and permission metadata.
#[async_trait::async_trait]
pub trait Registry: Send + Sync {
    /// Agent identity used by permission logging.
    fn name(&self) -> &str;
    /// Configured permission entry, if present.
    fn permission(&self, name: &str) -> Option<&PermissionAction>;
    /// Return a registered tool by name.
    fn tool(&self, name: &str) -> Option<Arc<dyn Tool>>;
    /// List registered names in the original registry order.
    fn names(&self) -> Vec<&str>;
    /// Evaluate the existing invocation policy before executing a known tool.
    async fn blocked(name: &str, arguments: &Value) -> Option<ToolResult>;
}
