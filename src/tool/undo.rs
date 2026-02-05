use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoInput {
    /// Number of commits to undo (default: 1)
    #[serde(default = "default_steps")]
    pub steps: usize,
    /// Show what would be undone without actually doing it
    #[serde(default)]
    pub preview: bool,
}

fn default_steps() -> usize {
    1
}

pub struct UndoTool;

#[async_trait]
impl Tool for UndoTool {
    fn id(&self) -> &str {
        "undo"
    }

    fn name(&self) -> &str {
        "Undo"
    }

    fn description(&self) -> &str {
        "Undo the last AI-generated changes by reverting git commits"
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "steps": {
                    "type": "integer",
                    "minimum": 1,
                    "maximum": 10,
                    "description": "Number of commits to undo",
                    "default": 1
                },
                "preview": {
                    "type": "boolean",
                    "description": "Show what would be undone without actually doing it",
                    "default": false
                }
            }
        })
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let params: UndoInput = serde_json::from_value(input)?;
        
        // Get current directory
        let cwd = std::env::current_dir()?;
        
        // Check if we're in a git repository
        let status = Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .current_dir(&cwd)
            .status()?;
            
        if !status.success() {
            return Ok(ToolResult {
                output: "Error: Not in a git repository".to_string(),
                success: false,
                metadata: HashMap::new(),
            });
        }

        // Get the last N commits
        let log_output = Command::new("git")
            .args([
                "log",
                "--oneline",
                "--max-count",
                &params.steps.to_string(),
            ])
            .current_dir(&cwd)
            .output()?;

        if !log_output.status.success() {
            return Ok(ToolResult {
                output: "Error: Failed to get git log".to_string(),
                success: false,
                metadata: HashMap::new(),
            });
        }

        let commits = String::from_utf8_lossy(&log_output.stdout);
        if commits.trim().is_empty() {
            return Ok(ToolResult {
                output: "No commits found to undo".to_string(),
                success: false,
                metadata: HashMap::new(),
            });
        }

        let commit_count = commits.lines().count();
        let commit_list: Vec<String> = commits.lines().map(|s| s.to_string()).collect();

        if params.preview {
            let mut preview = format!("Would undo {} commit(s):\n\n", commit_count);
            for commit in &commit_list {
                preview.push_str(&format!("  {}\n", commit));
            }
            
            // Show what files would be affected
            let diff_output = Command::new("git")
                .args([
                    "diff",
                    &format!("HEAD~{}", params.steps),
                    "--name-only",
                ])
                .current_dir(&cwd)
                .output()?;
                
            if diff_output.status.success() {
                let files = String::from_utf8_lossy(&diff_output.stdout);
                if !files.trim().is_empty() {
                    preview.push_str("\nFiles that would be affected:\n");
                    for file in files.lines() {
                        preview.push_str(&format!("  {}\n", file));
                    }
                }
            }
            
            return Ok(ToolResult {
                output: preview,
                success: true,
                metadata: HashMap::new(),
            });
        }

        // Actually perform the undo
        let revert_output = Command::new("git")
            .args([
                "reset",
                "--hard",
                &format!("HEAD~{}", params.steps),
            ])
            .current_dir(&cwd)
            .output()?;

        if revert_output.status.success() {
            let mut result = format!("Successfully undid {} commit(s):\n\n", commit_count);
            for commit in &commit_list {
                result.push_str(&format!("  {}\n", commit));
            }
            
            Ok(ToolResult {
                output: result,
                success: true,
                metadata: HashMap::new(),
            })
        } else {
            let error = String::from_utf8_lossy(&revert_output.stderr);
            Ok(ToolResult {
                output: format!("Failed to undo commits: {}", error),
                success: false,
                metadata: HashMap::new(),
            })
        }
    }
}