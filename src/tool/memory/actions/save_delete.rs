//! Mutating memory actions: save and delete.

use super::super::super::ToolResult;
use super::super::{MemoryEntry, MemoryTool, scope};
use anyhow::Result;
use serde_json::Value;

impl MemoryTool {
    pub(in super::super) async fn execute_save(&self, args: Value) -> Result<ToolResult> {
        let content = args["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("content is required for 'save' action"))?;
        let tags: Vec<String> = args["tags"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let scope = scope::save(&args);
        let importance = args["importance"].as_u64().map(|v| v as u8).unwrap_or(3);
        let mut entry = MemoryEntry::new(content, tags).with_importance(importance);
        if let Some(s) = scope {
            entry = entry.with_scope(s);
        }
        let id = {
            let mut store = self.store.lock().await;
            store.add(entry)
        };
        self.persist().await?;
        Ok(ToolResult::success(format!(
            "Memory saved with ID: {id}\nImportance: {importance}/5"
        )))
    }

    pub(in super::super) async fn execute_delete(&self, args: Value) -> Result<ToolResult> {
        let id = args["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("id is required for 'delete' action"))?;
        let deleted = {
            let mut store = self.store.lock().await;
            store.delete(id)
        };
        if deleted {
            self.persist().await?;
            Ok(ToolResult::success(format!("Memory deleted: {id}")))
        } else {
            Ok(ToolResult::error(format!("Memory not found: {id}")))
        }
    }
}
