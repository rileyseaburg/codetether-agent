//! Dispatch a [`BackendChoice`] to the underlying tool implementation and
//! return its [`ToolResult`]. Each backend is invoked inline so the router
//! does not require access to the full `ToolRegistry`.

use anyhow::Result;
use codetether_search::arguments::enrich_args;

use crate::tool::{Tool, ToolResult, file, memory, search, webfetch, websearch};

use super::types::{Backend, BackendChoice};

/// Execute one backend choice and return its raw [`ToolResult`].
///
/// The `query` is forwarded when the router omits an explicit
/// `pattern`/`query` arg in `choice.args`.
pub async fn run_choice(choice: &BackendChoice, query: &str) -> Result<ToolResult> {
    let args = enrich_args(choice, query);
    match choice.backend {
        Backend::Grep => search::GrepTool::new().execute(args).await,
        Backend::Glob => file::GlobTool::new().execute(args).await,
        Backend::Websearch => websearch::WebSearchTool::new().execute(args).await,
        Backend::Webfetch => webfetch::WebFetchTool::new().execute(args).await,
        Backend::Memory => memory::MemoryTool::new().execute(args).await,
        Backend::Rlm => Ok(ToolResult::error(
            "rlm backend requires a provider; wire RlmTool before requesting semantic search",
        )),
    }
}
