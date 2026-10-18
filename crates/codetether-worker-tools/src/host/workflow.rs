//! Model and workflow tool constructors bound to application execution.
use codetether_provider_core::Provider;
use codetether_tool_core::Tool;
use std::sync::Arc;
/// Workflow constructors requiring application execution bindings.
/// # Implementors
/// The worker binds MCP, RLM, Ralph and background go execution.
pub trait Workflow {
    /// Construct the application MCP bridge.
    fn mcp_bridge() -> Arc<dyn Tool>;
    /// Construct RLM with the original default configuration.
    fn rlm(provider: Arc<dyn Provider>, model: String) -> Arc<dyn Tool>;
    /// Construct Ralph with the supplied provider and model.
    fn ralph(provider: Arc<dyn Provider>, model: String) -> Arc<dyn Tool>;
    /// Construct go with the supplied optional completion callback.
    fn go(callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>) -> Arc<dyn Tool>;
}
