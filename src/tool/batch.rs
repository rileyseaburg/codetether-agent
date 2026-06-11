//! Batch Tool - Execute multiple tool calls in parallel.

#[path = "batch_policy.rs"]
mod batch_policy;
#[path = "batch_summary.rs"]
mod batch_summary;

use super::{Tool, ToolRegistry, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::{Arc, RwLock, Weak};

/// BatchTool executes multiple tool calls in parallel.
/// Uses lazy registry initialization to break circular dependency.
pub struct BatchTool {
    registry: Arc<RwLock<Option<Weak<ToolRegistry>>>>,
}

impl Default for BatchTool {
    fn default() -> Self {
        Self::new()
    }
}

impl BatchTool {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(None)),
        }
    }

    /// Set the registry after construction to break circular dependency.
    pub fn set_registry(&self, registry: Weak<ToolRegistry>) {
        let mut guard = self.registry.write().unwrap_or_else(|e| e.into_inner());
        *guard = Some(registry);
    }
}

#[derive(Deserialize)]
struct Params {
    calls: Vec<BatchCall>,
}

#[derive(Deserialize)]
struct BatchCall {
    #[serde(alias = "name")]
    tool: String,
    #[serde(default, alias = "arguments", alias = "params")]
    args: Value,
}

#[async_trait]
impl Tool for BatchTool {
    fn id(&self) -> &str {
        "batch"
    }
    fn name(&self) -> &str {
        "Batch Execute"
    }
    fn description(&self) -> &str {
        "Execute at most three short independent tool calls in parallel. Do not use for large file writes, long shell scripts, or broad search batches; call tools individually instead."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "calls": {
                    "type": "array",
                    "description": "Small array of tool calls to execute. Keep arguments short and use at most three calls. Preferred keys are `tool` + `args`; aliases `name` + `arguments` are also accepted for compatibility.",
                    "items": {
                        "type": "object",
                        "properties": {
                            "tool": {"type": "string", "description": "Tool ID to call (alias: `name`)"},
                            "args": {"type": "object", "description": "Arguments for the tool (alias: `arguments`)"},
                            "name": {"type": "string", "description": "Alias for `tool`"},
                            "arguments": {"type": "object", "description": "Alias for `args`"}
                        },
                        "anyOf": [
                            { "required": ["tool", "args"] },
                            { "required": ["name", "arguments"] }
                        ]
                    }
                }
            },
            "required": ["calls"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        if p.calls.is_empty() {
            return Ok(ToolResult::error("No calls provided"));
        }

        // Get registry from weak reference
        let registry = {
            let guard = self.registry.read().unwrap_or_else(|e| e.into_inner());
            match guard.as_ref() {
                Some(weak) => match weak.upgrade() {
                    Some(arc) => arc,
                    None => return Ok(ToolResult::error("Registry no longer available")),
                },
                None => return Ok(ToolResult::error("Registry not initialized")),
            }
        };

        // Execute all calls in parallel
        let futures: Vec<_> = p
            .calls
            .iter()
            .enumerate()
            .map(|(i, call)| {
                let tool_id = call.tool.clone();
                let args = call.args.clone();
                let registry = Arc::clone(&registry);

                async move { batch_policy::execute(i, tool_id, args, registry).await }
            })
            .collect();

        let results = futures::future::join_all(futures).await;

        Ok(batch_summary::build(results))
    }
}

#[cfg(test)]
mod tests {
    use super::Params;

    #[test]
    fn batch_call_accepts_name_arguments_aliases() {
        let params: Params = serde_json::from_value(serde_json::json!({
            "calls": [
                {
                    "name": "read",
                    "arguments": { "path": "src/main.rs" }
                }
            ]
        }))
        .expect("should parse alias form");

        assert_eq!(params.calls.len(), 1);
        assert_eq!(params.calls[0].tool, "read");
        assert_eq!(params.calls[0].args["path"], "src/main.rs");
    }

    #[test]
    fn batch_call_accepts_tool_args_primary_form() {
        let params: Params = serde_json::from_value(serde_json::json!({
            "calls": [
                {
                    "tool": "read",
                    "args": { "path": "src/main.rs" }
                }
            ]
        }))
        .expect("should parse primary form");

        assert_eq!(params.calls.len(), 1);
        assert_eq!(params.calls[0].tool, "read");
        assert_eq!(params.calls[0].args["path"], "src/main.rs");
    }
}
