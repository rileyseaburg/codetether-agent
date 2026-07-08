//! Single-entry retrieval action: get.

use super::super::super::ToolResult;
use super::super::MemoryTool;
use anyhow::Result;
use serde_json::Value;

impl MemoryTool {
    pub(in super::super) async fn execute_get(&self, args: Value) -> Result<ToolResult> {
        let id = args["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("id is required for 'get' action"))?;
        let entry = {
            let mut store = self.store.lock().await;
            store.get(id)
        };
        match entry {
            Some(e) => {
                self.persist().await?;
                Ok(ToolResult::success(format!(
                    "Memory ID: {}\nImportance: {}/5\nTags: {}\nCreated: {}\nAccessed: {} times\n\n{}",
                    e.id,
                    e.importance,
                    e.tags.join(", "),
                    e.created_at.format("%Y-%m-%d %H:%M:%S"),
                    e.access_count,
                    e.content
                )))
            }
            None => Ok(ToolResult::error(format!("Memory not found: {id}"))),
        }
    }
}
