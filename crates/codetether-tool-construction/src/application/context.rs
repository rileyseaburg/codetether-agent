//! Context-tool construction using caller-owned session, LSP and embedding types.
//!
//! [create] selects the existing shared tools without owning application state.
use crate::NativeTool;
use codetether_tool_core::Tool;
use std::sync::Arc;
/// Construct a context tool with the application's existing runtime bindings.
/// # Arguments
/// * `kind` - Context tool requested by the catalog.
/// # Returns
/// The selected tool, or `None` when another constructor group handles the kind.
/// # Examples
/// ```no_run
/// # fn example<H, S, E>() where H: codetether_lsp_tool::Host,
/// # S: codetether_context_browse::BrowseSource + codetether_context_budget::BudgetSource
/// # + codetether_context_pin::PinSource + codetether_context_summarize::SummarySource + 'static,
/// # E: codetether_memory_tool::AutoEmbedder {
/// let tool = codetether_tool_construction::application::context::create::<H, S, E>(
///     codetether_tool_construction::NativeTool::ContextBudget);
/// assert!(tool.is_some());
/// # }
/// ```
pub fn create<H, S, E>(kind: NativeTool) -> Option<Arc<dyn Tool>>
where
    H: codetether_lsp_tool::Host,
    S: codetether_context_browse::BrowseSource
        + codetether_context_budget::BudgetSource
        + codetether_context_pin::PinSource
        + codetether_context_summarize::SummarySource
        + 'static,
    E: codetether_memory_tool::AutoEmbedder,
{
    Some(match kind {
        NativeTool::Lsp => Arc::new(codetether_lsp_tool::LspTool::<H>::with_root(
            std::env::current_dir()
                .map(|p| format!("file://{}", p.display()))
                .unwrap_or_default(),
        )) as Arc<dyn Tool>,
        NativeTool::ContextBrowse => {
            Arc::new(codetether_context_browse::ContextBrowseTool::<S>::new()) as Arc<dyn Tool>
        }
        NativeTool::ContextBudget => {
            Arc::new(codetether_context_budget::ContextBudgetTool::<S>::new()) as Arc<dyn Tool>
        }
        NativeTool::ContextPin => {
            Arc::new(codetether_context_pin::ContextPinTool::<S>::new()) as Arc<dyn Tool>
        }
        NativeTool::ContextSummarizeCached => {
            Arc::new(codetether_context_summarize::ContextSummarizeTool::<S>::cached_only())
                as Arc<dyn Tool>
        }
        NativeTool::Memory => {
            Arc::new(codetether_memory_tool::Runtime::<E>::new()) as Arc<dyn Tool>
        }
        _ => return None,
    })
}
