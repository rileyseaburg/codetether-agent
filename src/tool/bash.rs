//! Bash tool: execute shell commands

use super::bash_github::load_github_command_auth;
use super::bash_identity::git_identity_env_from_tool_args;
use super::sandbox::{SandboxPolicy, execute_sandboxed};
use super::{Tool, ToolResult};
use crate::audit::{AuditCategory, AuditOutcome, try_audit_log};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Instant;
use tokio::process::Command;
use tokio::time::{Duration, timeout};

use crate::telemetry::{TOOL_EXECUTIONS, ToolExecution, record_persistent};
use crate::util;

/// Execute shell commands
pub struct BashTool {
    timeout_secs: u64,
    /// When true, execute commands through the sandbox with restricted env.
    sandboxed: bool,
    default_cwd: Option<PathBuf>,
}

impl BashTool {
    pub fn new() -> Self {
        let sandboxed = std::env::var("CODETETHER_SANDBOX_BASH")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        Self {
            timeout_secs: 120,
            sandboxed,
            default_cwd: None,
        }
    }

    pub fn with_cwd(default_cwd: PathBuf) -> Self {
        Self {
            default_cwd: Some(default_cwd),
            ..Self::new()
        }
    }

    /// Create a new BashTool with a custom timeout
    #[allow(dead_code)]
    pub fn with_timeout(timeout_secs: u64) -> Self {
        Self {
            timeout_secs,
            sandboxed: false,
            default_cwd: None,
        }
    }

    /// Create a sandboxed BashTool
    #[allow(dead_code)]
    pub fn sandboxed() -> Self {
        Self {
            timeout_secs: 120,
            sandboxed: true,
            default_cwd: None,
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

fn redact_output(mut output: String, secrets: &[String]) -> String {
    for secret in secrets {
        if !secret.is_empty() {
            output = output.replace(secret, "[REDACTED]");
        }
    }
    output
}

fn codetether_wrapped_command(command: &str) -> String {
    format!(
        "codetether() {{ \"$CODETETHER_BIN\" \"$@\"; }}\nexport -f codetether >/dev/null 2>&1 || true\n{command}"
    )
}

fn codetether_runtime_env() -> Option<(String, OsString)> {
    let current_exe = std::env::current_exe().ok()?;
    let mut path_entries = current_exe
        .parent()
        .map(|parent| vec![parent.to_path_buf()])
        .unwrap_or_default();
    if let Some(existing_path) = std::env::var_os("PATH") {
        path_entries.extend(std::env::split_paths(&existing_path));
    }
    let path = std::env::join_paths(path_entries).ok()?;
    Some((current_exe.to_string_lossy().into_owned(), path))
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
        let cwd = args["cwd"].as_str().map(PathBuf::from);
        let effective_cwd = cwd.clone().or_else(|| self.default_cwd.clone());
        let timeout_secs = args["timeout"].as_u64().unwrap_or(self.timeout_secs);
        let wrapped_command = codetether_wrapped_command(command);

        if let Some(reason) = interactive_auth_risk_reason(command) {
            // Log warning but don't block anymore per user request
            tracing::warn!("Interactive auth risk detected: {}", reason);
        }

        // Sandboxed execution path: restricted env, resource limits, audit logged
        if self.sandboxed {
            let policy = SandboxPolicy {
                allowed_paths: effective_cwd.clone().map(|d| vec![d]).unwrap_or_default(),
                allow_network: false,
                allow_exec: true,
                timeout_secs,
                ..SandboxPolicy::default()
            };
            let work_dir = effective_cwd.as_deref();
            let sandbox_result = execute_sandboxed(
                "bash",
                &["-c".to_string(), wrapped_command.clone()],
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
                        format!("bash:{}", util::truncate_bytes_safe(&command, 80)),
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
                    TOOL_EXECUTIONS.record(exec.success);
                    let data = serde_json::json!({
                        "tool": "bash",
                        "command": command,
                        "success": r.success,
                        "exit_code": r.exit_code,
                    });
                    let _ = record_persistent("tool_execution", &data);

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
                    TOOL_EXECUTIONS.record(exec.success);
                    let data = serde_json::json!({
                        "tool": "bash",
                        "command": command,
                        "success": false,
                        "error": e.to_string(),
                    });
                    let _ = record_persistent("tool_execution", &data);
                    Ok(ToolResult::error(format!("Sandbox error: {}", e)))
                }
            };
        }

        let mut cmd = Command::new("bash");
        cmd.arg("-c")
            .arg(&wrapped_command)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GCM_INTERACTIVE", "never")
            .env("DEBIAN_FRONTEND", "noninteractive")
            .env("SUDO_ASKPASS", "/bin/false")
            .env("SSH_ASKPASS", "/bin/false");
        if let Some((codetether_bin, path)) = codetether_runtime_env() {
            cmd.env("CODETETHER_BIN", codetether_bin).env("PATH", path);
        }
        for (key, value) in git_identity_env_from_tool_args(&args) {
            cmd.env(key, value);
        }
        for (arg_key, env_key) in [
            ("__ct_current_model", "CODETETHER_CURRENT_MODEL"),
            ("__ct_provenance_id", "CODETETHER_PROVENANCE_ID"),
            ("__ct_origin", "CODETETHER_ORIGIN"),
            ("__ct_agent_name", "CODETETHER_AGENT_NAME"),
            ("__ct_agent_identity_id", "CODETETHER_AGENT_IDENTITY_ID"),
            ("__ct_key_id", "CODETETHER_KEY_ID"),
            ("__ct_signature", "CODETETHER_SIGNATURE"),
            ("__ct_tenant_id", "CODETETHER_TENANT_ID"),
            ("__ct_worker_id", "CODETETHER_WORKER_ID"),
            ("__ct_session_id", "CODETETHER_SESSION_ID"),
            ("__ct_task_id", "CODETETHER_TASK_ID"),
            ("__ct_run_id", "CODETETHER_RUN_ID"),
            ("__ct_attempt_id", "CODETETHER_ATTEMPT_ID"),
        ] {
            if let Some(value) = args[arg_key].as_str() {
                cmd.env(env_key, value);
            }
        }
        let github_auth = match load_github_command_auth(
            command,
            effective_cwd.as_deref().and_then(|dir| dir.to_str()),
        )
        .await
        {
            Ok(auth) => auth,
            Err(err) => {
                tracing::warn!(error = %err, "Failed to load GitHub auth for bash command");
                None
            }
        };
        if let Some(auth) = github_auth.as_ref() {
            for (key, value) in &auth.env {
                cmd.env(key, value);
            }
        }

        if let Some(dir) = effective_cwd.as_deref() {
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
                let combined = redact_output(
                    combined,
                    github_auth
                        .as_ref()
                        .map(|auth| auth.redactions.as_slice())
                        .unwrap_or(&[]),
                );

                let success = output.status.success();

                if !success && looks_like_auth_prompt(&combined) {
                    tracing::warn!("Interactive auth prompt detected in output");
                }

                // Truncate if too long
                let max_len = 50_000;
                let (output_str, truncated) = if combined.len() > max_len {
                    // Find a valid char boundary at or before max_len
                    let truncate_at = match combined.is_char_boundary(max_len) {
                        true => max_len,
                        false => {
                            let mut boundary = max_len;
                            while !combined.is_char_boundary(boundary) && boundary > 0 {
                                boundary -= 1;
                            }
                            boundary
                        }
                    };
                    let truncated_output = format!(
                        "{}...\n[Output truncated, {} bytes total]",
                        &combined[..truncate_at],
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
                        "cwd": effective_cwd
                            .as_ref()
                            .map(|dir| dir.display().to_string()),
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
                TOOL_EXECUTIONS.record(exec.success);
                let _ = record_persistent(
                    "tool_execution",
                    &serde_json::to_value(&exec).unwrap_or_default(),
                );

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
                TOOL_EXECUTIONS.record(exec.success);
                let _ = record_persistent(
                    "tool_execution",
                    &serde_json::to_value(&exec).unwrap_or_default(),
                );

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
                TOOL_EXECUTIONS.record(exec.success);
                let _ = record_persistent(
                    "tool_execution",
                    &serde_json::to_value(&exec).unwrap_or_default(),
                );

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
            default_cwd: None,
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
            default_cwd: None,
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

    #[test]
    fn wraps_commands_with_codetether_function() {
        let wrapped = codetether_wrapped_command("codetether run 'hi'");
        assert!(wrapped.contains("codetether()"));
        assert!(wrapped.contains("CODETETHER_BIN"));
        assert!(wrapped.ends_with("codetether run 'hi'"));
    }
}
