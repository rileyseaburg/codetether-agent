use crate::provider::Provider;
use crate::tool::{ToolRegistry, ralph, rlm};
use std::sync::Arc;

/// Register model-backed mutable tools.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::worker_tool_registry::register_model_tools;
/// use codetether_agent::provider::openai::OpenAIProvider;
/// use codetether_agent::tool::ToolRegistry;
/// use std::sync::Arc;
///
/// let mut registry = ToolRegistry::new();
/// let provider = OpenAIProvider::new("test-key".to_string()).expect("provider");
/// register_model_tools(&mut registry, Arc::new(provider), "openai/gpt-4o-mini".to_string());
///
/// assert!(registry.contains("rlm"));
/// assert!(registry.contains("ralph"));
/// ```
pub fn register_model_tools(
    registry: &mut ToolRegistry,
    provider: Arc<dyn Provider>,
    model: String,
) {
    registry.register(Arc::new(rlm::RlmTool::new(
        Arc::clone(&provider),
        model.clone(),
        crate::rlm::RlmConfig::default(),
    )));
    registry.register(Arc::new(ralph::RalphTool::with_provider(provider, model)));
}
