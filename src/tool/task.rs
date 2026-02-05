//! Task Tool - Spawn sub-tasks for parallel or sequential execution.

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static TASK_COUNTER: AtomicUsize = AtomicUsize::new(1);

lazy_static::lazy_static! {
    static ref TASK_STORE: RwLock<HashMap<String, TaskInfo>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Clone)]
struct TaskInfo {
    id: String,
    description: String,
    status: TaskStatus,
    result: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TaskStatus {
    Pending,
    #[allow(dead_code)]
    Running,
    Complete,
    Failed,
}

pub struct TaskTool;

impl Default for TaskTool {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Deserialize)]
struct Params {
    action: String, // create, status, complete, list, cancel
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    result: Option<String>,
}

#[async_trait]
impl Tool for TaskTool {
    fn id(&self) -> &str {
        "task"
    }
    fn name(&self) -> &str {
        "Task Manager"
    }
    fn description(&self) -> &str {
        "Manage sub-tasks: create, query status, complete, or list tasks for tracking complex workflows."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["create", "status", "complete", "list", "cancel"],
                    "description": "Action to perform"
                },
                "id": {"type": "string", "description": "Task ID (for status/complete/cancel)"},
                "description": {"type": "string", "description": "Task description (for create)"},
                "result": {"type": "string", "description": "Task result (for complete)"}
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "create" => {
                let description = p
                    .description
                    .ok_or_else(|| anyhow::anyhow!("description required"))?;
                let id = format!("task_{}", TASK_COUNTER.fetch_add(1, Ordering::SeqCst));

                let task = TaskInfo {
                    id: id.clone(),
                    description: description.clone(),
                    status: TaskStatus::Pending,
                    result: None,
                };

                TASK_STORE.write().insert(id.clone(), task);
                Ok(
                    ToolResult::success(format!("Created task: {} - {}", id, description))
                        .with_metadata("task_id", json!(id)),
                )
            }
            "status" => {
                let id = p.id.ok_or_else(|| anyhow::anyhow!("id required"))?;
                let store = TASK_STORE.read();

                match store.get(&id) {
                    Some(task) => {
                        let status_str = match task.status {
                            TaskStatus::Pending => "pending",
                            TaskStatus::Running => "running",
                            TaskStatus::Complete => "complete",
                            TaskStatus::Failed => "failed",
                        };
                        Ok(ToolResult::success(format!(
                            "Task {} [{}]: {}\n{}",
                            task.id,
                            status_str,
                            task.description,
                            task.result.as_deref().unwrap_or("")
                        )))
                    }
                    None => Ok(ToolResult::error(format!("Task not found: {}", id))),
                }
            }
            "complete" => {
                let id = p.id.ok_or_else(|| anyhow::anyhow!("id required"))?;
                let result = p.result.unwrap_or_else(|| "Completed".to_string());

                let mut store = TASK_STORE.write();
                match store.get_mut(&id) {
                    Some(task) => {
                        task.status = TaskStatus::Complete;
                        task.result = Some(result.clone());
                        Ok(ToolResult::success(format!("Completed task: {}", id)))
                    }
                    None => Ok(ToolResult::error(format!("Task not found: {}", id))),
                }
            }
            "cancel" => {
                let id = p.id.ok_or_else(|| anyhow::anyhow!("id required"))?;

                let mut store = TASK_STORE.write();
                match store.get_mut(&id) {
                    Some(task) => {
                        task.status = TaskStatus::Failed;
                        task.result = Some("Cancelled".to_string());
                        Ok(ToolResult::success(format!("Cancelled task: {}", id)))
                    }
                    None => Ok(ToolResult::error(format!("Task not found: {}", id))),
                }
            }
            "list" => {
                let store = TASK_STORE.read();
                if store.is_empty() {
                    return Ok(ToolResult::success("No active tasks".to_string()));
                }

                let output = store
                    .values()
                    .map(|t| {
                        let icon = match t.status {
                            TaskStatus::Pending => "○",
                            TaskStatus::Running => "◐",
                            TaskStatus::Complete => "●",
                            TaskStatus::Failed => "✗",
                        };
                        format!("{} {} - {}", icon, t.id, t.description)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                Ok(ToolResult::success(output).with_metadata("count", json!(store.len())))
            }
            _ => Ok(ToolResult::error(format!("Unknown action: {}", p.action))),
        }
    }
}
