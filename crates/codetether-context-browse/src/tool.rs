//! Model-facing browse metadata and execution entry point.
use crate::{BrowseSource, ContextBrowseTool};
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
#[async_trait]
impl<S: BrowseSource> Tool for ContextBrowseTool<S> {
    fn id(&self) -> &str {
        "context_browse"
    }

    fn name(&self) -> &str {
        "ContextBrowse"
    }

    fn description(&self) -> &str {
        "BROWSE YOUR OWN HISTORY AS A FILESYSTEM (Meta-Harness, \
         arXiv:2603.28052). Materializes real files under \
         `.codetether-agent/history/<session-id>/turn-NNNN-<role>.md` \
         — one per turn in the canonical transcript — and returns the \
         body of any specific turn on request. Use only when the user allows \
         history access and user-designated repository sources are insufficient. \
         Distinct from `session_recall` (RLM-summarised archive) and \
         `memory` (curated notes). Actions: `list` (default) or \
         `show_turn` with an integer `turn`."
    }
    fn parameters(&self) -> Value {
        super::schema::parameters()
    }
    async fn execute(&self, args: Value) -> Result<ToolResult> {
        super::execute::run::<S>(args).await
    }
}
