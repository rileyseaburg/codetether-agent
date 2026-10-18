//! Bind provider-backed catalog construction to application tool types.
use super::*;
use codetether_tool_construction::provider::ProviderTool;
pub(super) fn create(
    kind: ProviderTool,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<dyn Tool> {
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
