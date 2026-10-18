//! Select a provider-backed factory without changing source ownership rules.
use super::ProviderTool;
use codetether_provider_core::Provider;
use codetether_tool_core::Tool;
use codetether_tool_registry::catalog::ToolKind;
use std::sync::Arc;

/// Construct a provider-backed tool for a scalar catalog kind.
///
/// # Arguments
/// * `kind` — Kind requested by the ordered catalog.
/// * `source` — Provider and model retained until Ralph consumes them.
/// * `factory` — Called once for a supported provider-backed kind.
/// # Returns
/// A constructed tool, or `None` for an unrelated kind. Unrelated kinds do
/// not inspect or modify the source and do not invoke the factory.
/// # Panics
/// Panics if a provider-backed kind is requested without a source.
/// # Examples
/// ```
/// use codetether_tool_construction::provider::create;
/// use codetether_tool_registry::catalog::ToolKind;
/// let mut source = None;
/// assert!(create(ToolKind::Read, &mut source, |_, _, _| unreachable!()).is_none());
/// assert!(source.is_none());
/// ```
pub fn create(
    kind: ToolKind,
    source: &mut Option<(Arc<dyn Provider>, String)>,
    factory: impl FnOnce(ProviderTool, Arc<dyn Provider>, String) -> Arc<dyn Tool>,
) -> Option<Arc<dyn Tool>> {
    let kind = match kind {
        ToolKind::ContextSummarizeProvider => ProviderTool::ContextSummarize,
        ToolKind::Rlm => ProviderTool::Rlm,
        ToolKind::SessionRecall => ProviderTool::SessionRecall,
        ToolKind::RalphProvider => ProviderTool::Ralph,
        _ => return None,
    };
    let (provider, model) = match kind {
        ProviderTool::ContextSummarize | ProviderTool::Rlm | ProviderTool::SessionRecall => {
            let (provider, model) = source.as_ref().expect("provider catalog mode");
            (Arc::clone(provider), model.clone())
        }
        ProviderTool::Ralph => source.take().expect("provider catalog mode"),
    };
    Some(factory(kind, provider, model))
}
