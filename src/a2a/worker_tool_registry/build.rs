use super::{register_mutating_tools, register_safe_tools};
use crate::a2a::worker::AutoApprove;
use crate::provider::Provider;
use crate::tool::{ToolRegistry, invalid};
use std::path::Path;
use std::sync::Arc;

/// Build the worker's tool registry for the current workspace and approval mode.
///
/// # Arguments
///
/// * `provider` - The provider used by model-backed tools.
/// * `model` - The resolved model identifier.
/// * `auto_approve` - The active worker approval mode.
/// * `workspace_dir` - The workspace root the tools should operate inside.
/// * `completion_callback` - Optional callback for background tool completion.
///
/// # Returns
///
/// Returns a [`ToolRegistry`] scoped to the worker policy and workspace root.
///
/// # Examples
///
/// ```rust
/// use anyhow::Result;
/// use async_trait::async_trait;
/// use codetether_agent::a2a::worker::AutoApprove;
/// use codetether_agent::a2a::worker_tool_registry::create_filtered_registry;
/// use codetether_agent::provider::{
///     CompletionRequest, CompletionResponse, FinishReason, ModelInfo, Provider, StreamChunk, Usage,
/// };
/// use futures::stream::{self, BoxStream};
/// use std::sync::Arc;
///
/// struct ExampleProvider;
///
/// #[async_trait]
/// impl Provider for ExampleProvider {
///     fn name(&self) -> &str { "example" }
///     async fn list_models(&self) -> Result<Vec<ModelInfo>> { Ok(vec![]) }
///     async fn complete(&self, _request: CompletionRequest) -> Result<CompletionResponse> {
///         Ok(CompletionResponse {
///             message: codetether_agent::provider::Message {
///                 role: codetether_agent::provider::Role::Assistant,
///                 content: vec![],
///             },
///             usage: Usage::default(),
///             finish_reason: FinishReason::Stop,
///         })
///     }
///     async fn complete_stream(
///         &self,
///         _request: CompletionRequest,
///     ) -> Result<BoxStream<'static, StreamChunk>> {
///         Ok(Box::pin(stream::empty()))
///     }
/// }
///
/// let registry = create_filtered_registry(
///     Arc::new(ExampleProvider),
///     "example/model".to_string(),
///     AutoApprove::Safe,
///     std::path::Path::new("."),
///     None,
/// );
///
/// assert!(registry.contains("read"));
/// assert!(!registry.contains("write"));
/// ```
pub fn create_filtered_registry(
    provider: Arc<dyn Provider>,
    model: String,
    auto_approve: AutoApprove,
    workspace_dir: &Path,
    completion_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    let root_path = workspace_dir.to_path_buf();
    register_safe_tools(&mut registry, workspace_dir, &root_path);
    if matches!(auto_approve, AutoApprove::All) {
        register_mutating_tools(
            &mut registry,
            provider,
            model,
            root_path,
            completion_callback,
        );
    }
    registry.register(Arc::new(invalid::InvalidTool::new()));
    registry
}
