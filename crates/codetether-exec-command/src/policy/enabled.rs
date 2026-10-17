//! Decide whether an invocation retains sandbox confinement.
use super::*;

/// Decide whether an invocation should retain sandbox confinement.
///
/// # Arguments
/// Uses effective configuration, command classification and invocation authority.
/// # Returns
/// Whether the sandbox should be enabled for this invocation.
pub fn enabled(config: &Config, command: &str, args: &Value) -> bool {
    if matches!(
        config.effective_sandbox_mode(),
        SandboxMode::DangerFullAccess
    ) || env::truthy("CODETETHER_UNSANDBOXED_BASH")
        || env::is_false("CODETETHER_SANDBOX_BASH")
        || codetether_runtime_policy::command::is_read_only_command(command)
    {
        return false;
    }
    let unavailable = codetether_sandbox_runtime::unavailable_reason().is_some();
    let approved =
        codetether_runtime_policy::approval_gate::approved_or_session_command("exec_command", args);
    if escalated(args) && approved {
        return false;
    }
    !(unavailable && !codetether_sandbox_runtime::direct_fallback_env_allowed() && approved)
}
