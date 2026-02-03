//! Batch Tool - Execute multiple tool calls in parallel.

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::{json, Value};
use super::{Tool, ToolResult, ToolRegistry};
use std::sync::{Arc, RwLock, Weak};

/// BatchTool executes multiple tool calls in parallel.
/// Uses lazy registry initialization to break circular dependency.
pub struct BatchTool {
    registry: Arc<RwLock<Option<Weak<ToolRegistry>>>>,
}

impl BatchTool {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(None)),
        }
    }

    /// Set the registry after construction to break circular dependency.
    pub fn set_registry(&self, registry: Weak<ToolRegistry>) {
        let mut guard = self.registry.write().unwrap();
        *guard = Some(registry);
    }
}

#[derive(Deserialize)]
struct Params {
    calls: Vec<BatchCall>,
}

#[derive(Deserialize)]
struct BatchCall {
    tool: String,
    args: Value,
}

#[async_trait]
impl Tool for BatchTool {
    fn id(&self) -> &str { "batch" }
    fn name(&self) -> &str { "Batch Execute" }
    fn description(&self) -> &str { "Execute multiple tool calls in parallel. Each call specifies a tool name and arguments." }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "calls": {
                    "type": "array",
                    "description": "Array of tool calls to execute",
                    "items": {
                        "type": "object",
                        "properties": {
                            "tool": {"type": "string", "description": "Tool ID to call"},
                            "args": {"type": "object", "description": "Arguments for the tool"}
                        },
                        "required": ["tool", "args"]
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
            let guard = self.registry.read().unwrap();
            match guard.as_ref() {
                Some(weak) => match weak.upgrade() {
                    Some(arc) => arc,
                    None => return Ok(ToolResult::error("Registry no longer available")),
                },
                None => return Ok(ToolResult::error("Registry not initialized")),
            }
        };
        
        // Execute all calls in parallel
        let futures: Vec<_> = p.calls.iter().enumerate().map(|(i, call)| {
            let tool_id = call.tool.clone();
            let args = call.args.clone();
            let registry = Arc::clone(&registry);
            
            async move {
                // Prevent recursive batch calls
                if tool_id == "batch" {
                    return (i, tool_id, ToolResult::error("Cannot call batch from within batch"));
                }
                
                match registry.get(&tool_id) {
                    Some(tool) => {
                        match tool.execute(args).await {
                            Ok(result) => (i, tool_id, result),
                            Err(e) => (i, tool_id, ToolResult::error(format!("Error: {}", e))),
                        }
                    }
                    None => {
                        // Use the invalid tool handler for better error messages
                        let available_tools = registry.list().iter().map(|s| s.to_string()).collect();
                        let invalid_tool = super::invalid::InvalidTool::with_context(tool_id.clone(), available_tools);
                        let invalid_args = serde_json::json!({
                            "requested_tool": tool_id,
                            "args": args
                        });
                        match invalid_tool.execute(invalid_args).await {
                            Ok(result) => (i, tool_id.clone(), result),
                            Err(e) => (i, tool_id.clone(), ToolResult::error(format!("Unknown tool: {}. Error: {}", tool_id, e))),
                        }
                    }
                }
            }
        }).collect();
        
        let results = futures::future::join_all(futures).await;
        
        let mut output_parts = Vec::new();
        let mut success_count = 0;
        let mut error_count = 0;
        
        for (idx, tool_id, result) in results {
            if result.success {
                success_count += 1;
                output_parts.push(format!("[{}] ✓ {}:\n{}", idx + 1, tool_id, result.output));
            } else {
                error_count += 1;
                output_parts.push(format!("[{}] ✗ {}:\n{}", idx + 1, tool_id, result.output));
            }
        }
        
        let summary = format!("Batch complete: {} succeeded, {} failed\n\n{}", 
            success_count, error_count, output_parts.join("\n\n"));
        
        let overall_success = error_count == 0;
        if overall_success {
            Ok(ToolResult::success(summary).with_metadata("success_count", json!(success_count)))
        } else {
            Ok(ToolResult::error(summary).with_metadata("error_count", json!(error_count)))
        }
    }
}
