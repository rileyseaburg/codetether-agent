//! Swarm Share Tool - Publish and query shared results between sub-agents
//!
//! This tool allows sub-agents in a swarm to share intermediate results,
//! enabling real-time collaboration between concurrent agents.

use super::{Tool, ToolResult};
use crate::swarm::result_store::ResultStore;
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

pub struct SwarmShareTool {
    store: Arc<ResultStore>,
    /// The subtask ID of the agent using this tool
    producer_id: String,
}

impl SwarmShareTool {
    pub fn new(store: Arc<ResultStore>, producer_id: String) -> Self {
        Self { store, producer_id }
    }

    /// Create with default ResultStore and empty producer ID (for registry registration)
    pub fn with_defaults() -> Self {
        Self {
            store: ResultStore::new_arc(),
            producer_id: String::new(),
        }
    }
}

impl Default for SwarmShareTool {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[derive(Deserialize)]
struct Params {
    action: String,
    #[serde(default)]
    key: Option<String>,
    #[serde(default)]
    value: Option<Value>,
    #[serde(default)]
    tags: Option<Vec<String>>,
    #[serde(default)]
    prefix: Option<String>,
}

#[async_trait]
impl Tool for SwarmShareTool {
    fn id(&self) -> &str {
        "swarm_share"
    }

    fn name(&self) -> &str {
        "Swarm Share"
    }

    fn description(&self) -> &str {
        "Share results with other sub-agents in the swarm. Actions: publish (share a result), \
         get (retrieve a result by key), query_tags (find results by tags), \
         query_prefix (find results by key prefix), list (show all shared results)."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["publish", "get", "query_tags", "query_prefix", "list"],
                    "description": "Action to perform"
                },
                "key": {
                    "type": "string",
                    "description": "Result key (for publish/get)"
                },
                "value": {
                    "description": "Result value to publish (any JSON value)"
                },
                "tags": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Tags for publish or query_tags"
                },
                "prefix": {
                    "type": "string",
                    "description": "Key prefix for query_prefix"
                },
                "producer": {
                    "type": "string",
                    "description": "Producer subtask ID to filter by (for query)"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "publish" => {
                let key = p
                    .key
                    .ok_or_else(|| anyhow::anyhow!("key required for publish"))?;
                let value = p
                    .value
                    .ok_or_else(|| anyhow::anyhow!("value required for publish"))?;
                let tags = p.tags.unwrap_or_default();

                let result = self
                    .store
                    .publish(&key, &self.producer_id, value, tags, None)
                    .await?;

                Ok(ToolResult::success(format!(
                    "Published result '{}' (type: {})",
                    key, result.schema.type_name
                )))
            }
            "get" => {
                let key = p
                    .key
                    .ok_or_else(|| anyhow::anyhow!("key required for get"))?;

                match self.store.get(&key).await {
                    Some(result) => {
                        let output = json!({
                            "key": result.key,
                            "producer": result.producer_id,
                            "value": result.value,
                            "type": result.schema.type_name,
                            "tags": result.tags,
                            "published_at": result.published_at.to_rfc3339(),
                        });
                        Ok(ToolResult::success(
                            serde_json::to_string_pretty(&output)
                                .unwrap_or_else(|_| format!("{:?}", result.value)),
                        ))
                    }
                    None => Ok(ToolResult::error(format!("No result found for key: {key}"))),
                }
            }
            "query_tags" => {
                let tags = p
                    .tags
                    .ok_or_else(|| anyhow::anyhow!("tags required for query_tags"))?;

                let results = self.store.query_by_tags(&tags).await;
                let output: Vec<Value> = results
                    .iter()
                    .map(|r| {
                        json!({
                            "key": r.key,
                            "producer": r.producer_id,
                            "type": r.schema.type_name,
                            "tags": r.tags,
                        })
                    })
                    .collect();

                Ok(ToolResult::success(format!(
                    "Found {} results matching tags {:?}:\n{}",
                    output.len(),
                    tags,
                    serde_json::to_string_pretty(&output).unwrap_or_default()
                )))
            }
            "query_prefix" => {
                let prefix = p
                    .prefix
                    .ok_or_else(|| anyhow::anyhow!("prefix required for query_prefix"))?;

                let results = self.store.query_by_prefix(&prefix).await;
                let output: Vec<Value> = results
                    .iter()
                    .map(|r| {
                        json!({
                            "key": r.key,
                            "producer": r.producer_id,
                            "type": r.schema.type_name,
                            "tags": r.tags,
                        })
                    })
                    .collect();

                Ok(ToolResult::success(format!(
                    "Found {} results with prefix '{}':\n{}",
                    output.len(),
                    prefix,
                    serde_json::to_string_pretty(&output).unwrap_or_default()
                )))
            }
            "list" => {
                let results = self.store.get_all().await;
                if results.is_empty() {
                    return Ok(ToolResult::success(
                        "No shared results in store".to_string(),
                    ));
                }

                let output: Vec<Value> = results
                    .iter()
                    .map(|r| {
                        json!({
                            "key": r.key,
                            "producer": r.producer_id,
                            "type": r.schema.type_name,
                            "tags": r.tags,
                        })
                    })
                    .collect();

                Ok(ToolResult::success(format!(
                    "{} shared results:\n{}",
                    output.len(),
                    serde_json::to_string_pretty(&output).unwrap_or_default()
                )))
            }
            _ => Ok(ToolResult::error(format!("Unknown action: {}", p.action))),
        }
    }
}
