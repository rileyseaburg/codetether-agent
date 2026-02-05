//! Bash tool: execute shell commands

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

/// Execute shell commands
pub struct BashTool {
    timeout_secs: u64,
}

impl BashTool {
    pub fn new() -> Self {
        Self { timeout_secs: 120 }
    }

    /// Create a new BashTool with a custom timeout
    #[allow(dead_code)]
    pub fn with_timeout(timeout_secs: u64) -> Self {
        Self { timeout_secs }
    }
}

#[async_trait]
impl Tool for BashTool {
    fn id(&self) -> &str {
        "bash"
    }

    fn name(&self) -> &str {
        "Bash"
    }

    fn description(&self) -> &str {
        "bash(command: string, cwd?: string, timeout?: int) - Execute a shell command. Commands run in a bash shell with the current working directory."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "description": "The shell command to execute"
                },
                "cwd": {
                    "type": "string",
                    "description": "Working directory for the command (optional)"
                },
                "timeout": {
                    "type": "integer",
                    "description": "Timeout in seconds (default: 120)"
                }
            },
            "required": ["command"],
            "example": {
                "command": "ls -la src/",
                "cwd": "/path/to/project"
            }
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let command = match args["command"].as_str() {
            Some(c) => c,
            None => {
                return Ok(ToolResult::structured_error(
                    "INVALID_ARGUMENT",
                    "bash",
                    "command is required",
                    Some(vec!["command"]),
                    Some(json!({"command": "ls -la", "cwd": "."})),
                ));
            }
        };
        let cwd = args["cwd"].as_str();
        let timeout_secs = args["timeout"].as_u64().unwrap_or(self.timeout_secs);

        let mut cmd = Command::new("bash");
        cmd.arg("-c")
            .arg(command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let result = timeout(Duration::from_secs(timeout_secs), cmd.output()).await;

        match result {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                let exit_code = output.status.code().unwrap_or(-1);

                let combined = if stderr.is_empty() {
                    stdout.to_string()
                } else if stdout.is_empty() {
                    stderr.to_string()
                } else {
                    format!("{}\n--- stderr ---\n{}", stdout, stderr)
                };

                let success = output.status.success();

                // Truncate if too long
                let max_len = 50_000;
                let (output_str, truncated) = if combined.len() > max_len {
                    let truncated_output = format!(
                        "{}...\n[Output truncated, {} bytes total]",
                        &combined[..max_len],
                        combined.len()
                    );
                    (truncated_output, true)
                } else {
                    (combined, false)
                };

                Ok(ToolResult {
                    output: output_str,
                    success,
                    metadata: [
                        ("exit_code".to_string(), json!(exit_code)),
                        ("truncated".to_string(), json!(truncated)),
                    ]
                    .into_iter()
                    .collect(),
                })
            }
            Ok(Err(e)) => Ok(ToolResult::structured_error(
                "EXECUTION_FAILED",
                "bash",
                &format!("Failed to execute command: {}", e),
                None,
                Some(json!({"command": command})),
            )),
            Err(_) => Ok(ToolResult::structured_error(
                "TIMEOUT",
                "bash",
                &format!("Command timed out after {} seconds", timeout_secs),
                None,
                Some(json!({
                    "command": command,
                    "hint": "Consider increasing timeout or breaking into smaller commands"
                })),
            )),
        }
    }
}

impl Default for BashTool {
    fn default() -> Self {
        Self::new()
    }
}
