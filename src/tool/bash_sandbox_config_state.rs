//! Bash sandbox enablement state helpers.

use serde_json::Value;

pub(super) fn enabled_for_args(sandbox_enabled: bool, command: &str, args: &Value) -> bool {
    enabled_for_state(
        sandbox_enabled,
        crate::runtime_policy::is_read_only_command(command),
        crate::tool::sandbox::unavailable_reason().is_some(),
        crate::tool::sandbox::direct_fallback_env_allowed(),
        crate::runtime_policy::approved_or_session_command("bash", args),
    )
}

pub(super) async fn approved_direct_fallback(
    default_enabled: bool,
    command: &str,
    args: &Value,
) -> bool {
    let sandbox_enabled = super::enabled(default_enabled).await;
    !enabled_for_args(sandbox_enabled, command, args)
        && enabled_for_read_class(sandbox_enabled, command)
}

pub(super) fn enabled_for_read_class(sandbox_enabled: bool, command: &str) -> bool {
    sandbox_enabled && !crate::runtime_policy::is_read_only_command(command)
}

pub(super) fn enabled_for_state(
    sandbox_enabled: bool,
    read_only: bool,
    sandbox_unavailable: bool,
    env_allows_direct: bool,
    approved_direct: bool,
) -> bool {
    sandbox_enabled && !read_only && !(sandbox_unavailable && !env_allows_direct && approved_direct)
}
