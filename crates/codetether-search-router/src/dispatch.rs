//! Construct concrete search backends using the caller's memory factory.
use anyhow::Result;
use codetether_file_tools as file;
use codetether_grep_tool as search;
use codetether_memory_tool::AutoEmbedder;
use codetether_search::{Backend, BackendChoice, arguments::enrich_args};
use codetether_tool_core::{Tool, ToolResult};
use codetether_webfetch as webfetch;
use codetether_websearch as websearch;

/// Execute one backend choice and return its raw [`ToolResult`].
///
/// The `query` is forwarded when the router omits an explicit
/// `pattern`/`query` arg in `choice.args`.
/// # Arguments
/// * `choice` - Selected backend and its arguments.
/// * `query` - Original query used to fill omitted search arguments.
/// # Returns
/// The selected tool's result.
/// # Errors
/// Returns errors raised by the backend tool.
/// # Examples
/// ```rust,no_run
/// # async fn example(choice: &codetether_search::BackendChoice)->anyhow::Result<()> {
/// let _result=codetether_search_router::dispatch::run_choice::<codetether_memory_tool::NoAutoEmbedder>(choice,"main").await?;
/// # Ok(()) }
/// ```
pub async fn run_choice<B: AutoEmbedder>(
    choice: &BackendChoice,
    query: &str,
) -> Result<ToolResult> {
    let args = enrich_args(choice, query);
    match choice.backend {
        Backend::Grep => search::GrepTool::new().execute(args).await,
        Backend::Glob => file::GlobTool::new().execute(args).await,
        Backend::Websearch => websearch::WebSearchTool::new().execute(args).await,
        Backend::Webfetch => webfetch::WebFetchTool::new().execute(args).await,
        Backend::Memory => {
            codetether_memory_tool::Runtime::<B>::new()
                .execute(args)
                .await
        }
        Backend::Rlm => Ok(ToolResult::error(
            "rlm backend requires a provider; wire RlmTool before requesting semantic search",
        )),
    }
}
