//! Bash tool: execute shell commands

use super::sandbox::{SandboxPolicy, execute_sandboxed};
use super::{Tool, ToolResult};
use crate::audit::{AuditCategory, AuditOutcome, try_audit_log};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::process::Stdio;
use std::time::Instant;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

use crate::telemetry::{TOOL_EXECUTIONS, ToolExecution, record_persistent};

/// Execute shell commands
pub struct BashTool {
    timeout_secs: u64,
    /// When true, execute commands through the sandbox with restricted env.
    sandboxed: bool,
}

impl BashTool {
    pub fn new() -> Self {
        let sandboxed = std::env::var("CODETETHER_SANDBOX_BASH")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        Self {
            timeout_secs: 120,
            sandboxed,
        }
    }

    /// Create a new BashTool with a custom timeout
    #[allow(dead_code)]
    pub fn with_timeout(timeout_secs: u64) -> Self {
        Self {
            timeout_secs,
            sandboxed: false,
        }
    }

    /// Create a sandboxed BashTool
    #[allow(dead_code)]
    pub fn sandboxed() -> Self {
        Self {
            timeout_secs: 120,
            sandboxed: true,
        }
    }
}

fn interactive_auth_risk_reason(command: &str) -> Option<&'static str> {
    let lower = command.to_ascii_lowercase();

    let has_sudo = lower.starts_with("sudo ")
        || lower.contains(";sudo ")
        || lower.contains("&& sudo ")
        || lower.contains("|| sudo ")
        || lower.contains("| sudo ");
    let sudo_non_interactive =
        lower.contains("sudo -n") || lower.contains("sudo --non-interactive");
    if has_sudo && !sudo_non_interactive {
        return Some("Command uses sudo without non-interactive mode (-n).");
    }

    let has_ssh_family = lower.starts_with("ssh ")
        || lower.contains(";ssh ")
        || lower.starts_with("scp ")
        || lower.contains(";scp ")
        || lower.starts_with("sftp ")
        || lower.contains(";sftp ")
        || lower.contains(" rsync ");
    if has_ssh_family && !lower.contains("batchmode=yes") {
        return Some(
            "SSH-family command may prompt for password/passphrase (missing -o BatchMode=yes).",
        );
    }

    if lower.starts_with("su ")
        || lower.contains(";su ")
        || lower.contains(" passwd ")
        || lower.starts_with("passwd")
        || lower.contains("ssh-add")
    {
        return Some("Command is interactive and may require a password prompt.");
    }

    None
}

fn looks_like_auth_prompt(output: &str) -> bool {
    let lower = output.to_ascii_lowercase();
    [
        "[sudo] password for",
        "password:",
        "passphrase",
        "no tty present and no askpass program specified",
        "a terminal is required to read the password",
        "could not read password",
        "permission denied (publickey,password",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
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
        let exec_start = Instant::now();

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

        if let Some(reason) = interactive_auth_risk_reason(command) {
            // Log warning but don't block anymore per user request
            tracing::warn!("Interactive auth risk detected: {}", reason);
        }

        // Sandboxed execution path: restricted env, resource limits, audit logged
        if self.sandboxed {
            let policy = SandboxPolicy {
                allowed_paths: cwd
                    .map(|d| vec![std::path::PathBuf::from(d)])
                    .unwrap_or_default(),
                allow_network: false,
                allow_exec: true,
                timeout_secs,
                ..SandboxPolicy::default()
            };
            let work_dir = cwd.map(std::path::Path::new);
            let sandbox_result = execute_sandboxed(
                "bash",
                &["-c".to_string(), command.to_string()],
                &policy,
                work_dir,
            )
            .await;

            // Audit log the sandboxed execution
            if let Some(audit) = try_audit_log() {
                let (outcome, detail) = match &sandbox_result {
                    Ok(r) => (
                        if r.success {
                            AuditOutcome::Success
                        } else {
                            AuditOutcome::Failure
                        },
                        json!({
                            "sandboxed": true,
                            "exit_code": r.exit_code,
                            "duration_ms": r.duration_ms,
                            "violations": r.sandbox_violations,
                        }),
                    ),
                    Err(e) => (
                        AuditOutcome::Failure,
                        json!({ "sandboxed": true, "error": e.to_string() }),
                    ),
                };
                audit
                    .log(
                        AuditCategory::Sandbox,
                        format!("bash:{}", &command[..command.len().min(80)]),
                        outcome,
                        None,
                        Some(detail),
                    )
                    .await;
            }

            return match sandbox_result {
                Ok(r) => {
                    let duration = exec_start.elapsed();
                    let exec = ToolExecution::start(
                        "bash",
                        json!({ "command": command, "sandboxed": true }),
                    );
                    let exec = if r.success {
                        exec.complete_success(format!("exit_code={:?}", r.exit_code), duration)
                    } else {
                        exec.complete_error(format!("exit_code={:?}", r.exit_code), duration)
                    };
                    TOOL_EXECUTIONS.record(exec.clone());
                    record_persistent(exec);

                    Ok(ToolResult {
                        output: r.output,
                        success: r.success,
                        metadata: [
                            ("exit_code".to_string(), json!(r.exit_code)),
                            ("sandboxed".to_string(), json!(true)),
                            (
                                "sandbox_violations".to_string(),
                                json!(r.sandbox_violations),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    })
                }
                Err(e) => {
                    let duration = exec_start.elapsed();
                    let exec = ToolExecution::start(
                        "bash",
                        json!({ "command": command, "sandboxed": true }),
                    )
                    .complete_error(e.to_string(), duration);
                    TOOL_EXECUTIONS.record(exec.clone());
                    record_persistent(exec);
                    Ok(ToolResult::error(format!("Sandbox error: {}", e)))
                }
            };
        }

        let mut cmd = Command::new("bash");
        cmd.arg("-c")
            .arg(command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GCM_INTERACTIVE", "never")
            .env("DEBIAN_FRONTEND", "noninteractive")
            .env("SUDO_ASKPASS", "/bin/false")
            .env("SSH_ASKPASS", "/bin/false");

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

                if !success && looks_like_auth_prompt(&combined) {
                    tracing::warn!("Interactive auth prompt detected in output");
                }

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
                    (combined.clone(), false)
                };

                let duration = exec_start.elapsed();

                // Record telemetry
                let exec = ToolExecution::start(
                    "bash",
                    json!({
                        "command": command,
                        "cwd": cwd,
                        "timeout": timeout_secs,
                    }),
                );
                let exec = if success {
                    exec.complete_success(
                        format!("exit_code={}, output_len={}", exit_code, combined.len()),
                        duration,
                    )
                } else {
                    exec.complete_error(
                        format!(
                            "exit_code={}: {}",
                            exit_code,
                            combined.lines().next().unwrap_or("(no output)")
                        ),
                        duration,
                    )
                };
                TOOL_EXECUTIONS.record(exec.clone());
                record_persistent(exec);

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
            Ok(Err(e)) => {
                let duration = exec_start.elapsed();
                let exec = ToolExecution::start(
                    "bash",
                    json!({
                        "command": command,
                        "cwd": cwd,
                    }),
                )
                .complete_error(format!("Failed to execute: {}", e), duration);
                TOOL_EXECUTIONS.record(exec.clone());
                record_persistent(exec);

                Ok(ToolResult::structured_error(
                    "EXECUTION_FAILED",
                    "bash",
                    &format!("Failed to execute command: {}", e),
                    None,
                    Some(json!({"command": command})),
                ))
            }
            Err(_) => {
                let duration = exec_start.elapsed();
                let exec = ToolExecution::start(
                    "bash",
                    json!({
                        "command": command,
                        "cwd": cwd,
                    }),
                )
                .complete_error(format!("Timeout after {}s", timeout_secs), duration);
                TOOL_EXECUTIONS.record(exec.clone());
                record_persistent(exec);

                Ok(ToolResult::structured_error(
                    "TIMEOUT",
                    "bash",
                    &format!("Command timed out after {} seconds", timeout_secs),
                    None,
                    Some(json!({
                        "command": command,
                        "hint": "Consider increasing timeout or breaking into smaller commands"
                    })),
                ))
            }
        }
    }
}

impl Default for BashTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sandboxed_bash_basic() {
        let tool = BashTool {
            timeout_secs: 10,
            sandboxed: true,
        };
        let result = tool
            .execute(json!({ "command": "echo hello sandbox" }))
            .await
            .unwrap();
        assert!(result.success);
        assert!(result.output.contains("hello sandbox"));
        assert_eq!(result.metadata.get("sandboxed"), Some(&json!(true)));
    }

    #[tokio::test]
    async fn sandboxed_bash_timeout() {
        let tool = BashTool {
            timeout_secs: 1,
            sandboxed: true,
        };
        let result = tool
            .execute(json!({ "command": "sleep 30" }))
            .await
            .unwrap();
        assert!(!result.success);
    }

    #[test]
    fn detects_interactive_auth_risk() {
        assert!(interactive_auth_risk_reason("sudo apt update").is_some());
        assert!(interactive_auth_risk_reason("ssh user@host").is_some());
        assert!(interactive_auth_risk_reason("sudo -n apt update").is_none());
        assert!(interactive_auth_risk_reason("ssh -o BatchMode=yes user@host").is_none());
    }

    #[test]
    fn detects_auth_prompt_output() {
        assert!(looks_like_auth_prompt("[sudo] password for riley:"));
        assert!(looks_like_auth_prompt(
            "sudo: a terminal is required to read the password"
        ));
        assert!(!looks_like_auth_prompt("command completed successfully"));
    }
}
