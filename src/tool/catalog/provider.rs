//! Bind provider-backed catalog construction to application tool types.
use super::*;
use codetether_tool_construction::provider::{self, ProviderTool};
pub(super) fn register(host: &mut Native<'_>, kind: ToolKind) -> bool {
    let Some(tool) = provider::create(kind, &mut host.provider, create) else {
        return false;
    };
    host.registry.register(tool);
    true
}
fn create(kind: ProviderTool, provider: Arc<dyn Provider>, model: String) -> Arc<dyn Tool> {
    match kind {
        ProviderTool::ContextSummarize => Arc::new(context_summarize::ContextSummarizeTool::new(
            provider, model,
        )),
        ProviderTool::Rlm => Arc::new(rlm::RlmTool::new(
            provider,
            model,
            crate::rlm::RlmConfig::default(),
        )),
        ProviderTool::SessionRecall => Arc::new(session_recall::SessionRecallTool::new(
            provider,
            model,
            crate::rlm::RlmConfig::default(),
        )),
        ProviderTool::Ralph => Arc::new(ralph::RalphTool::with_provider(provider, model)),
    }
}
