use super::{register_edit_tools, register_go_tool, register_model_tools, register_task_tools};
use crate::provider::Provider;
use crate::tool::ToolRegistry;
use std::path::PathBuf;
use std::sync::Arc;

/// Register the worker's state-changing tools against the current workspace root.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_tool_registry::register_mutating_tools;
/// use codetether_agent::provider::openai::OpenAIProvider;
/// use codetether_agent::tool::ToolRegistry;
/// use std::path::PathBuf;
/// use std::sync::Arc;
///
/// let mut registry = ToolRegistry::new();
/// let provider = OpenAIProvider::new("test-key".to_string()).expect("provider");
/// register_mutating_tools(
///     &mut registry,
///     Arc::new(provider),
///     "openai/gpt-4o-mini".to_string(),
///     PathBuf::from("."),
///     None,
/// );
///
/// assert!(registry.contains("write"));
/// assert!(registry.contains("undo"));
/// ```
pub fn register_mutating_tools(
    registry: &mut ToolRegistry,
    provider: Arc<dyn Provider>,
    model: String,
    root_path: PathBuf,
    completion_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) {
    register_edit_tools(registry, root_path);
    register_task_tools(registry);
    register_model_tools(registry, provider, model);
    register_go_tool(registry, completion_callback);
}
