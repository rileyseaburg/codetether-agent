//! Construct provider-backed tools using caller-owned session and host types.
//!
//! [create] retains the selected provider and model in the requested tool.
use super::ProviderTool;
use codetether_provider_core::Provider;
use codetether_tool_core::Tool;
use std::sync::Arc;
/// Construct one provider-backed catalog tool.
/// # Arguments
/// * `kind` - Tool selected by the ordered catalog.
/// * `provider` - Existing provider handle for the tool.
/// * `model` - Existing model selection.
/// # Returns
/// The requested tool with its original default RLM configuration, if applicable.
/// # Examples
/// ```no_run
/// # fn example<S, R, Q, A>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>)
/// # where S: codetether_context_summarize::SummarySource + 'static,
/// # R: codetether_rlm_tool::Host + 'static, Q: codetether_session_recall::Host + 'static,
/// # A: codetether_ralph_tool::Host + 'static {
/// let _tool = codetether_tool_construction::provider::bound::create::<S, R, Q, A>(
///     codetether_tool_construction::provider::ProviderTool::Rlm, provider, "model".into());
/// # }
/// ```
pub fn create<S, R, Q, A>(
    kind: ProviderTool,
    provider: Arc<dyn Provider>,
    model: String,
) -> Arc<dyn Tool>
where
    S: codetether_context_summarize::SummarySource + 'static,
    R: codetether_rlm_tool::Host + 'static,
    Q: codetether_session_recall::Host + 'static,
    A: codetether_ralph_tool::Host + 'static,
{
    match kind {
        ProviderTool::ContextSummarize => {
            Arc::new(codetether_context_summarize::ContextSummarizeTool::<S>::new(provider, model))
        }
        ProviderTool::Rlm => Arc::new(codetether_rlm_tool::RlmTool::<R>::new(
            provider,
            model,
            codetether_rlm::RlmConfig::default(),
        )),
        ProviderTool::SessionRecall => {
            Arc::new(codetether_session_recall::SessionRecallTool::<Q>::new(
                provider,
                model,
                codetether_rlm::RlmConfig::default(),
            ))
        }
        ProviderTool::Ralph => Arc::new(codetether_ralph_tool::RalphTool::<A>::with_provider(
            provider, model,
        )),
    }
}
