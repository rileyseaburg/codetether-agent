//! Search action.

use super::super::super::ToolResult;
use super::super::{MemoryTool, scope};
use anyhow::Result;
use serde_json::Value;

impl MemoryTool {
    pub(in super::super) async fn execute_search(&self, args: Value) -> Result<ToolResult> {
        let query = args["query"].as_str();
        let tags: Option<Vec<String>> = args["tags"].as_array().map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        });
        let limit = args["limit"].as_u64().map(|v| v as usize).unwrap_or(10);
        let scope = scope::search(&args);
        let results = {
            let mut store = self.store.lock().await;
            store.search(query, tags.as_deref(), scope.as_deref(), limit)
        };
        if results.is_empty() {
            return Ok(ToolResult::success(
                "No memories found matching your criteria.".to_string(),
            ));
        }
        let output = results
            .iter()
            .enumerate()
            .map(|(i, m)| {
                format!(
                    "{}. [{}] {} - {}\n   Tags: {}\n   Created: {}",
                    i + 1,
                    m.id,
                    m.content.chars().take(80).collect::<String>()
                        + if m.content.len() > 80 { "..." } else { "" },
                    format!("accessed {} times", m.access_count),
                    m.tags.join(", "),
                    m.created_at.format("%Y-%m-%d %H:%M")
                )
            })
            .collect::<Vec<_>>()
            .join("\n\n");
        Ok(ToolResult::success(format!(
            "Found {} memories:\n\n{output}",
            results.len()
        )))
    }
}
