//! Reporting actions: tags and stats.

use super::super::super::ToolResult;
use super::super::MemoryTool;
use anyhow::Result;
use serde_json::Value;

impl MemoryTool {
    pub(in super::super) async fn execute_tags(&self, _args: Value) -> Result<ToolResult> {
        let tags = {
            let store = self.store.lock().await;
            store.all_tags()
        };
        if tags.is_empty() {
            return Ok(ToolResult::success(
                "No tags yet. Add tags when saving memories.".to_string(),
            ));
        }
        let mut sorted: Vec<_> = tags.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        let output = sorted
            .iter()
            .map(|(tag, count)| format!("  {tag} ({count} memories)"))
            .collect::<Vec<_>>()
            .join("\n");
        Ok(ToolResult::success(format!("Available tags:\n\n{output}")))
    }

    pub(in super::super) async fn execute_stats(&self, _args: Value) -> Result<ToolResult> {
        let stats = {
            let store = self.store.lock().await;
            store.stats()
        };
        let tags_output = if stats.tags.is_empty() {
            "None".to_string()
        } else {
            let mut sorted: Vec<_> = stats.tags.iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(a.1));
            sorted
                .iter()
                .take(10)
                .map(|(t, c)| format!("  {t}: {c}"))
                .collect::<Vec<_>>()
                .join("\n")
        };
        Ok(ToolResult::success(format!(
            "Memory Statistics:\n\nTotal entries: {}\nTotal accesses: {}\nUnique tags: {}\n\nTop tags:\n{}",
            stats.total_entries, stats.total_accesses, stats.unique_tags, tags_output
        )))
    }
}
