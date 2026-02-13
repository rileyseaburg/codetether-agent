//! Memory tool: Persistent knowledge capture and retrieval
//!
//! Allows agents to store important insights, learnings, and decisions
//! that persist across sessions for future reference.

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

/// A single memory entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    /// Unique identifier
    pub id: String,
    /// The memory content
    pub content: String,
    /// Tags for categorization/search
    pub tags: Vec<String>,
    /// When this memory was created
    pub created_at: DateTime<Utc>,
    /// When this memory was last accessed
    pub accessed_at: DateTime<Utc>,
    /// How many times this memory has been accessed
    pub access_count: u64,
    /// Optional project/context scope
    pub scope: Option<String>,
    /// Source of the memory (session_id, tool, etc.)
    pub source: Option<String>,
    /// Importance level (1-5)
    pub importance: u8,
}

impl MemoryEntry {
    pub fn new(content: impl Into<String>, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            content: content.into(),
            tags,
            created_at: now,
            accessed_at: now,
            access_count: 0,
            scope: None,
            source: None,
            importance: 3, // default medium importance
        }
    }

    pub fn with_scope(mut self, scope: impl Into<String>) -> Self {
        self.scope = Some(scope.into());
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn with_importance(mut self, importance: u8) -> Self {
        self.importance = importance.min(5);
        self
    }

    pub fn touch(&mut self) {
        self.accessed_at = Utc::now();
        self.access_count += 1;
    }
}

/// Memory store for persistence
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MemoryStore {
    entries: HashMap<String, MemoryEntry>,
}

impl MemoryStore {
    /// Get the default memory file path
    pub fn default_path() -> std::path::PathBuf {
        directories::ProjectDirs::from("com", "codetether", "codetether")
            .map(|p| p.data_dir().join("memory.json"))
            .unwrap_or_else(|| PathBuf::from(".codetether/memory.json"))
    }

    /// Load from disk
    pub async fn load() -> Result<Self> {
        let path = Self::default_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path).await?;
        let store: MemoryStore = serde_json::from_str(&content)?;
        Ok(store)
    }

    /// Save to disk
    pub async fn save(&self) -> Result<()> {
        let path = Self::default_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content).await?;
        Ok(())
    }

    /// Add a new memory
    pub fn add(&mut self, entry: MemoryEntry) -> String {
        let id = entry.id.clone();
        self.entries.insert(id.clone(), entry);
        id
    }

    /// Get a memory by ID
    pub fn get(&mut self, id: &str) -> Option<MemoryEntry> {
        let entry = self.entries.get_mut(id)?;
        entry.touch();
        Some(entry.clone())
    }

    /// Search memories by query or tags
    pub fn search(
        &mut self,
        query: Option<&str>,
        tags: Option<&[String]>,
        limit: usize,
    ) -> Vec<MemoryEntry> {
        let mut results: Vec<MemoryEntry> = self
            .entries
            .values_mut()
            .filter(|entry| {
                // Filter by tags if provided
                if let Some(search_tags) = tags {
                    if !search_tags.is_empty()
                        && !search_tags.iter().any(|t| entry.tags.contains(t))
                    {
                        return false;
                    }
                }

                // Filter by query if provided
                if let Some(q) = query {
                    let q_lower = q.to_lowercase();
                    let matches_content = entry.content.to_lowercase().contains(&q_lower);
                    let matches_tags = entry
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&q_lower));
                    if !matches_content && !matches_tags {
                        return false;
                    }
                }

                true
            })
            .map(|e| {
                e.touch();
                e.clone()
            })
            .collect();

        // Sort by importance (descending) then access_count (descending)
        results.sort_by(|a, b| {
            b.importance
                .cmp(&a.importance)
                .then_with(|| b.access_count.cmp(&a.access_count))
        });

        results.truncate(limit);
        results
    }

    /// Get all tags with counts
    pub fn all_tags(&self) -> HashMap<String, u64> {
        let mut tags: HashMap<String, u64> = HashMap::new();
        for entry in self.entries.values() {
            for tag in &entry.tags {
                *tags.entry(tag.clone()).or_insert(0) += 1;
            }
        }
        tags
    }

    /// Delete a memory
    pub fn delete(&mut self, id: &str) -> bool {
        self.entries.remove(id).is_some()
    }

    /// Get statistics
    pub fn stats(&self) -> MemoryStats {
        let total = self.entries.len();
        let total_accesses: u64 = self.entries.values().map(|e| e.access_count).sum();
        let tags = self.all_tags();
        MemoryStats {
            total_entries: total,
            total_accesses,
            unique_tags: tags.len(),
            tags,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_entries: usize,
    pub total_accesses: u64,
    pub unique_tags: usize,
    pub tags: HashMap<String, u64>,
}

/// Memory Tool - Store and retrieve persistent knowledge
pub struct MemoryTool {
    store: tokio::sync::Mutex<MemoryStore>,
}

impl Default for MemoryTool {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryTool {
    pub fn new() -> Self {
        Self {
            store: tokio::sync::Mutex::new(MemoryStore::default()),
        }
    }

    /// Initialize store from disk
    pub async fn init(&self) -> Result<()> {
        let mut store = self.store.lock().await;
        *store = MemoryStore::load().await?;
        Ok(())
    }

    /// Persist store to disk
    pub async fn persist(&self) -> Result<()> {
        let store = self.store.lock().await;
        store.save().await
    }
}

#[async_trait]
impl Tool for MemoryTool {
    fn id(&self) -> &str {
        "memory"
    }

    fn name(&self) -> &str {
        "Memory"
    }

    fn description(&self) -> &str {
        "Store and retrieve persistent knowledge across sessions. Use 'save' to capture important insights, 'search' to find relevant memories, 'list' to see all entries, 'tags' to see available categories, or 'delete' to remove an entry."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "description": "Action to perform: 'save' (store new memory), 'search' (find memories), 'get' (retrieve specific memory), 'list' (show recent), 'tags' (show categories), 'delete' (remove), 'stats' (show statistics)",
                    "enum": ["save", "search", "get", "list", "tags", "delete", "stats"]
                },
                "content": {
                    "type": "string",
                    "description": "Memory content to save (required for 'save' action)"
                },
                "tags": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Tags for categorization (optional for 'save')"
                },
                "query": {
                    "type": "string",
                    "description": "Search query (for 'search' action)"
                },
                "scope": {
                    "type": "string",
                    "description": "Project/context scope (optional for 'save')"
                },
                "importance": {
                    "type": "integer",
                    "description": "Importance level 1-5 (optional for 'save', default 3)"
                },
                "id": {
                    "type": "string",
                    "description": "Memory ID (required for 'get' and 'delete')"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum results to return (default 10, for 'search' and 'list')"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        // Ensure store is loaded
        self.init().await.ok();

        let action = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("action is required"))?;

        match action {
            "save" => self.execute_save(args).await,
            "search" => self.execute_search(args).await,
            "get" => self.execute_get(args).await,
            "list" => self.execute_list(args).await,
            "tags" => self.execute_tags(args).await,
            "delete" => self.execute_delete(args).await,
            "stats" => self.execute_stats(args).await,
            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Use 'save', 'search', 'get', 'list', 'tags', 'delete', or 'stats'.",
                action
            ))),
        }
    }
}

impl MemoryTool {
    async fn execute_save(&self, args: Value) -> Result<ToolResult> {
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

        let scope = args["scope"].as_str().map(String::from);
        let importance = args["importance"].as_u64().map(|v| v as u8).unwrap_or(3);

        let mut entry = MemoryEntry::new(content, tags).with_importance(importance);

        if let Some(s) = scope {
            entry = entry.with_scope(s);
        }

        let id = {
            let mut store = self.store.lock().await;
            store.add(entry)
        };

        // Persist to disk
        self.persist().await?;

        Ok(ToolResult::success(format!(
            "Memory saved with ID: {}\nImportance: {}/5",
            id, importance
        )))
    }

    async fn execute_search(&self, args: Value) -> Result<ToolResult> {
        let query = args["query"].as_str();
        let tags: Option<Vec<String>> = args["tags"].as_array().map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        });
        let limit = args["limit"].as_u64().map(|v| v as usize).unwrap_or(10);

        let tags_ref = tags.as_ref().map(|v| v.as_slice());

        let results = {
            let mut store = self.store.lock().await;
            store.search(query, tags_ref, limit)
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
                    m.id.chars().take(8).collect::<String>(),
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
            "Found {} memories:\n\n{}",
            results.len(),
            output
        )))
    }

    async fn execute_get(&self, args: Value) -> Result<ToolResult> {
        let id = args["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("id is required for 'get' action"))?;

        let entry = {
            let mut store = self.store.lock().await;
            store.get(id).map(|e| e.clone())
        };

        match entry {
            Some(e) => {
                // Persist the updated access count
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
            None => Ok(ToolResult::error(format!("Memory not found: {}", id))),
        }
    }

    async fn execute_list(&self, args: Value) -> Result<ToolResult> {
        let limit = args["limit"].as_u64().map(|v| v as usize).unwrap_or(10);

        let results = {
            let mut store = self.store.lock().await;
            store.search(None, None, limit)
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
                    m.id.chars().take(8).collect::<String>(),
                    m.content.chars().take(60).collect::<String>()
                        + if m.content.len() > 60 { "..." } else { "" },
                    m.importance,
                    m.access_count
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(ToolResult::success(format!(
            "Recent memories:\n\n{}",
            output
        )))
    }

    async fn execute_tags(&self, _args: Value) -> Result<ToolResult> {
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
            .map(|(tag, count)| format!("  {} ({} memories)", tag, count))
            .collect::<Vec<_>>()
            .join("\n");

        Ok(ToolResult::success(format!(
            "Available tags:\n\n{}",
            output
        )))
    }

    async fn execute_delete(&self, args: Value) -> Result<ToolResult> {
        let id = args["id"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("id is required for 'delete' action"))?;

        let deleted = {
            let mut store = self.store.lock().await;
            store.delete(id)
        };

        if deleted {
            self.persist().await?;
            Ok(ToolResult::success(format!("Memory deleted: {}", id)))
        } else {
            Ok(ToolResult::error(format!("Memory not found: {}", id)))
        }
    }

    async fn execute_stats(&self, _args: Value) -> Result<ToolResult> {
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
                .map(|(t, c)| format!("  {}: {}", t, c))
                .collect::<Vec<_>>()
                .join("\n")
        };

        Ok(ToolResult::success(format!(
            "Memory Statistics:\n\n\
             Total entries: {}\n\
             Total accesses: {}\n\
             Unique tags: {}\n\n\
             Top tags:\n{}",
            stats.total_entries, stats.total_accesses, stats.unique_tags, tags_output
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_memory_save_and_get() {
        let tool = MemoryTool::new();

        // Save a memory
        let result = tool
            .execute(json!({
                "action": "save",
                "content": "Test memory content",
                "tags": ["test", "example"],
                "importance": 4
            }))
            .await
            .unwrap();

        assert!(result.success);

        // List memories
        let result = tool
            .execute(json!({
                "action": "list",
                "limit": 5
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.output.contains("Test memory content"));

        // Get stats
        let result = tool
            .execute(json!({
                "action": "stats"
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.output.contains("Total entries: 1"));
    }

    #[tokio::test]
    async fn test_memory_search() {
        let tool = MemoryTool::new();

        // Save with specific tags
        tool.execute(json!({
            "action": "save",
            "content": "Rust programming insights",
            "tags": ["rust", "programming"]
        }))
        .await
        .unwrap();

        tool.execute(json!({
            "action": "save",
            "content": "Python tips",
            "tags": ["python", "programming"]
        }))
        .await
        .unwrap();

        // Search by tag
        let result = tool
            .execute(json!({
                "action": "search",
                "tags": ["rust"]
            }))
            .await
            .unwrap();

        assert!(result.success);
        assert!(result.output.contains("Rust"));
        assert!(!result.output.contains("Python"));
    }
}
