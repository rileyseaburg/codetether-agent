//! List action.

use super::super::super::ToolResult;
use super::super::{MemoryTool, scope};
use anyhow::Result;
use serde_json::Value;

impl MemoryTool {
    pub(in super::super) async fn execute_list(&self, args: Value) -> Result<ToolResult> {
        let limit = args["limit"].as_u64().map(|v| v as usize).unwrap_or(10);
        let scope = scope::search(&args);
        let results = {
            let mut store = self.store.lock().await;
            store.search(None, None, scope.as_deref(), limit)
        };
        if results.is_empty() {
            return Ok(ToolResult::success(
                "No memories stored yet. Use 'save' to add your first memory.".to_string(),
            ));
        }
        let output = results
            .iter()
            .enumerate()
            .map(|(i, m)| {
                format!(
                    "{}. [{}] {} (importance: {}/5, accessed: {}x)",
                    i + 1,
                    m.id,
                    m.content.chars().take(60).collect::<String>()
                        + if m.content.len() > 60 { "..." } else { "" },
                    m.importance,
                    m.access_count
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        Ok(ToolResult::success(format!("Recent memories:\n\n{output}")))
    }
}
