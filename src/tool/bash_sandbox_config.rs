use crate::config::{Config, SandboxMode};
use serde_json::Value;

#[path = "bash_sandbox_config_reason.rs"]
mod reason;
#[path = "bash_sandbox_config_state.rs"]
mod state;

pub(super) async fn enabled(default_enabled: bool) -> bool {
    match Config::load().await {
        Ok(config) => from_mode(default_enabled, config.effective_sandbox_mode()),
        Err(_) => default_enabled,
    }
}

pub(super) async fn enabled_for_args(default_enabled: bool, command: &str, args: &Value) -> bool {
    state::enabled_for_args(enabled(default_enabled).await, command, args)
}

pub(super) async fn unsafe_reason_for_args(
    default_enabled: bool,
    command: &str,
    args: &Value,
) -> &'static str {
    reason::for_args(default_enabled, command, args).await
}

pub(super) fn allow_network() -> bool {
    std::env::var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK")
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
#[path = "bash_sandbox_config_tests.rs"]
mod tests;
