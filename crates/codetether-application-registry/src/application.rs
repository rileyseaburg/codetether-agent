//! Concrete factories and policy supplied by the native application.
use crate::ToolRegistry;
use codetether_provider_core::Provider;
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::{future::Future, sync::Arc};
/// Application services needed to populate and authorize a tool registry.
/// # Implementors
/// Applications retain concrete tool construction, provider-registry ownership
/// and policy evaluation; shared registry mechanics stay in this crate.
pub trait Application: Sized + Send + Sync + 'static {
    /// Provider registry accepted by the application's routed search tool.
    type Providers: Send + Sync + 'static;
    /// Populate the ordered default or provider-backed tool catalog.
    fn populate(registry: &mut ToolRegistry<Self>, provider: Option<(Arc<dyn Provider>, String)>);
    /// Register routed search using an existing provider registry.
    fn register_search(registry: &mut ToolRegistry<Self>, providers: Arc<Self::Providers>);
    /// Construct routed search using the provider passed to shared initialization.
    fn register_provider_search(registry: &mut ToolRegistry<Self>, provider: Arc<dyn Provider>);
    /// Evaluate a nested batch invocation using the application's runtime policy.
    /// # Arguments
    /// Receives the original tool name and arguments.
    /// # Returns
    /// A blocking tool result, or `None` when execution is allowed.
    fn evaluate(tool: &str, args: &Value) -> impl Future<Output = Option<ToolResult>> + Send;
}
