//! Bash unsandboxed execution reason helpers.

use crate::config::SandboxMode;
use serde_json::Value;

pub(super) async fn unsafe_reason() -> &'static str {
    match crate::config::Config::load().await {
        Ok(config)
            if matches!(
                config.effective_sandbox_mode(),
                SandboxMode::DangerFullAccess
            ) =>
        {
            "config sandbox_mode=danger-full-access"
        }
        _ => super::super::bash_sandbox_mode::unsafe_reason(),
    }
}

pub(super) async fn for_command(command: &str) -> &'static str {
    if crate::runtime_policy::is_read_only_command(command) {
        "read_only_command"
    } else {
        unsafe_reason().await
    }
}

pub(super) async fn for_args(default_enabled: bool, command: &str, args: &Value) -> &'static str {
    if super::state::approved_direct_fallback(default_enabled, command, args).await {
        "approved_os_sandbox_unavailable_fallback"
    } else {
        for_command(command).await
    }
}
