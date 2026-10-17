//! Configured bash sandbox decisions, including approved direct fallback.
//! Use `enabled_for_args` for a tool call and `unsafe_reason_for_args` for its audit reason.

use codetether_config::{Config, SandboxMode};
use serde_json::Value;

#[path = "config_reason.rs"]
mod reason;
#[path = "config_state.rs"]
mod state;

/// Read the configured sandbox mode, falling back to the supplied default.
///
/// # Arguments
/// `default_enabled` is the tool construction default.
/// # Returns
/// Whether the configured mode permits sandboxing.
///
/// # Examples
/// ```rust
/// let _decision = codetether_bash_policy::config::enabled(true);
/// ```
pub async fn enabled(default_enabled: bool) -> bool {
    match Config::load().await {
        Ok(config) => from_mode(default_enabled, config.effective_sandbox_mode()),
        Err(_) => default_enabled,
    }
}

/// Resolve sandbox use for a command and its approval arguments.
///
/// # Arguments
/// `default_enabled` is the tool default; `command` and `args` identify the call.
/// # Returns
/// Whether this call must use the OS sandbox.
///
/// # Examples
/// ```rust
/// let args = serde_json::json!({"command": "date"});
/// let _decision = codetether_bash_policy::config::enabled_for_args(true, "date", &args);
/// ```
pub async fn enabled_for_args(default_enabled: bool, command: &str, args: &Value) -> bool {
    state::enabled_for_args(enabled(default_enabled).await, command, args)
}

/// Describe why the command runs directly for audit metadata.
///
/// # Arguments
/// `default_enabled` is the tool default; `command` and `args` identify the call.
/// # Returns
/// A stable reason for direct execution.
///
/// # Examples
/// ```rust
/// let args = serde_json::json!({"command": "date"});
/// let _reason = codetether_bash_policy::config::unsafe_reason_for_args(true, "date", &args);
/// ```
pub async fn unsafe_reason_for_args(
    default_enabled: bool,
    command: &str,
    args: &Value,
) -> &'static str {
    reason::for_args(default_enabled, command, args).await
}

/// Read the bash network override, then the shared network setting.
///
/// # Arguments
/// None.
/// # Returns
/// Whether network access is enabled.
///
/// # Examples
/// ```rust
/// let _network_allowed = codetether_bash_policy::config::allow_network();
/// ```
pub fn allow_network() -> bool {
    std::env::var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK")
        .or_else(|_| std::env::var("CODETETHER_ALLOW_NETWORK"))
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes"))
        .unwrap_or(false)
}

fn from_mode(default_enabled: bool, mode: SandboxMode) -> bool {
    match mode {
        SandboxMode::DangerFullAccess => false,
        SandboxMode::ReadOnly | SandboxMode::WorkspaceWrite => default_enabled,
    }
}

#[cfg(test)]
#[path = "config_tests.rs"]
mod tests;
