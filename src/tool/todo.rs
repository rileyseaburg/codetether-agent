//! Todo Tool - Read and write todo items for task tracking.

use super::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;

const TODO_FILE: &str = ".codetether-todos.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub content: String,
    pub status: TodoStatus,
    #[serde(default)]
    pub priority: Priority,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum TodoStatus {
    #[default]
    Pending,
    #[serde(alias = "in_progress")]
    #[serde(alias = "inprogress")]
    InProgress,
    Done,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
    Critical,
}

pub struct TodoReadTool {
    root: PathBuf,
}

pub struct TodoWriteTool {
    root: PathBuf,
}

impl Default for TodoReadTool {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TodoWriteTool {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoReadTool {
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    fn load_todos(&self) -> Result<Vec<TodoItem>> {
        let path = self.root.join(TODO_FILE);
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

impl TodoWriteTool {
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    fn load_todos(&self) -> Result<Vec<TodoItem>> {
        let path = self.root.join(TODO_FILE);
        if !path.exists() {
            return Ok(Vec::new());
        }
        let content = std::fs::read_to_string(&path)?;
        Ok(serde_json::from_str(&content)?)
    }

    fn save_todos(&self, todos: &[TodoItem]) -> Result<()> {
        let path = self.root.join(TODO_FILE);
        let content = serde_json::to_string_pretty(todos)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    fn generate_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        format!("todo_{}", now.as_millis())
    }
}

#[derive(Deserialize)]
struct ReadParams {
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    priority: Option<String>,
}

#[derive(Deserialize)]
struct WriteParams {
    action: String, // add, update, delete, clear
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    priority: Option<String>,
}

#[async_trait]
impl Tool for TodoReadTool {
    fn id(&self) -> &str {
        "todoread"
    }
    fn name(&self) -> &str {
        "Todo Read"
    }
    fn description(&self) -> &str {
        "Read todo items. Filter by status (pending/in_progress/done/blocked) or priority (low/medium/high/critical)."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "status": {"type": "string", "enum": ["pending", "in_progress", "inprogress", "done", "blocked"]},
                "priority": {"type": "string", "enum": ["low", "medium", "high", "critical"]}
            }
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: ReadParams = serde_json::from_value(params).unwrap_or(ReadParams {
            status: None,
            priority: None,
        });

        let todos = self.load_todos()?;

        let filtered: Vec<&TodoItem> = todos
            .iter()
            .filter(|t| {
                if let Some(ref status) = p.status {
                    let expected = match status.as_str() {
                        "pending" => TodoStatus::Pending,
                        "in_progress" | "inprogress" => TodoStatus::InProgress,
                        "done" => TodoStatus::Done,
                        "blocked" => TodoStatus::Blocked,
                        _ => return true,
                    };
                    if t.status != expected {
                        return false;
                    }
                }
                if let Some(ref priority) = p.priority {
                    let expected = match priority.as_str() {
                        "low" => Priority::Low,
                        "medium" => Priority::Medium,
                        "high" => Priority::High,
                        "critical" => Priority::Critical,
                        _ => return true,
                    };
                    if t.priority != expected {
                        return false;
                    }
                }
                true
            })
            .collect();

        if filtered.is_empty() {
            return Ok(ToolResult::success("No todos found".to_string()));
        }

        let output = filtered
            .iter()
            .map(|t| {
                let status_icon = match t.status {
                    TodoStatus::Pending => "○",
                    TodoStatus::InProgress => "◐",
                    TodoStatus::Done => "●",
                    TodoStatus::Blocked => "✗",
                };
                let priority_label = match t.priority {
                    Priority::Low => "[low]",
                    Priority::Medium => "",
                    Priority::High => "[HIGH]",
                    Priority::Critical => "[CRITICAL]",
                };
                format!("{} {} {} {}", status_icon, t.id, priority_label, t.content)
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(ToolResult::success(output).with_metadata("count", json!(filtered.len())))
    }
}

#[async_trait]
impl Tool for TodoWriteTool {
    fn id(&self) -> &str {
        "todowrite"
    }
    fn name(&self) -> &str {
        "Todo Write"
    }
    fn description(&self) -> &str {
        "Manage todo items: add, update, delete, or clear todos."
    }
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {"type": "string", "enum": ["add", "update", "delete", "clear"], "description": "Action to perform"},
                "id": {"type": "string", "description": "Todo ID (for update/delete)"},
                "content": {"type": "string", "description": "Todo content (for add/update)"},
                "status": {"type": "string", "enum": ["pending", "in_progress", "inprogress", "done", "blocked"]},
                "priority": {"type": "string", "enum": ["low", "medium", "high", "critical"]}
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: WriteParams = serde_json::from_value(params).context("Invalid params")?;
        let mut todos = self.load_todos()?;

        match p.action.as_str() {
            "add" => {
                let content = p
                    .content
                    .ok_or_else(|| anyhow::anyhow!("content required for add"))?;
                let status = p
                    .status
                    .map(|s| match s.as_str() {
                        "in_progress" | "inprogress" => TodoStatus::InProgress,
                        "done" => TodoStatus::Done,
                        "blocked" => TodoStatus::Blocked,
                        _ => TodoStatus::Pending,
                    })
                    .unwrap_or_default();
                let priority = p
                    .priority
                    .map(|s| match s.as_str() {
                        "low" => Priority::Low,
                        "high" => Priority::High,
                        "critical" => Priority::Critical,
                        _ => Priority::Medium,
                    })
                    .unwrap_or_default();

                let id = self.generate_id();
                todos.push(TodoItem {
                    id: id.clone(),
                    content,
                    status,
                    priority,
                    created_at: Some(chrono::Utc::now().to_rfc3339()),
                });
                self.save_todos(&todos)?;
                Ok(ToolResult::success(format!("Added todo: {}", id)))
            }
            "update" => {
                let id =
                    p.id.ok_or_else(|| anyhow::anyhow!("id required for update"))?;
                let todo = todos
                    .iter_mut()
                    .find(|t| t.id == id)
                    .ok_or_else(|| anyhow::anyhow!("Todo not found: {}", id))?;

                if let Some(content) = p.content {
                    todo.content = content;
                }
                if let Some(status) = p.status {
                    todo.status = match status.as_str() {
                        "in_progress" | "inprogress" => TodoStatus::InProgress,
                        "done" => TodoStatus::Done,
                        "blocked" => TodoStatus::Blocked,
                        _ => TodoStatus::Pending,
                    };
                }
                if let Some(priority) = p.priority {
                    todo.priority = match priority.as_str() {
                        "low" => Priority::Low,
                        "high" => Priority::High,
                        "critical" => Priority::Critical,
                        _ => Priority::Medium,
                    };
                }
                self.save_todos(&todos)?;
                Ok(ToolResult::success(format!("Updated todo: {}", id)))
            }
            "delete" => {
                let id =
                    p.id.ok_or_else(|| anyhow::anyhow!("id required for delete"))?;
                let len_before = todos.len();
                todos.retain(|t| t.id != id);
                if todos.len() == len_before {
                    return Ok(ToolResult::error(format!("Todo not found: {}", id)));
                }
                self.save_todos(&todos)?;
                Ok(ToolResult::success(format!("Deleted todo: {}", id)))
            }
            "clear" => {
                let count = todos.len();
                todos.clear();
                self.save_todos(&todos)?;
                Ok(ToolResult::success(format!("Cleared {} todos", count)))
            }
            _ => Ok(ToolResult::error(format!("Unknown action: {}", p.action))),
        }
    }
}
