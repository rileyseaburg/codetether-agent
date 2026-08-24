//! Bash sandbox enablement state helpers.

use serde_json::Value;

pub(super) fn enabled_for_args(sandbox_enabled: bool, command: &str, args: &Value) -> bool {
    let network = crate::tool::network_access::allowed_for(args);
    enabled_for_state(
        sandbox_enabled,
        crate::runtime_policy::is_read_only_command(command),
        crate::tool::sandbox::unavailable_reason_for(network).is_some(),
        crate::tool::sandbox::direct_fallback_env_allowed(),
        crate::runtime_policy::approved_receipt("bash", args),
    )
}

pub(super) async fn approved_direct_fallback(
    default_enabled: bool,
    command: &str,
    args: &Value,
) -> bool {
    let sandbox_enabled = super::enabled(default_enabled).await;
    !enabled_for_args(sandbox_enabled, command, args)
        && enabled_for_command_class(sandbox_enabled, command)
}

pub(super) fn enabled_for_command_class(sandbox_enabled: bool, _command: &str) -> bool {
    sandbox_enabled
}

pub(super) fn enabled_for_state(
    sandbox_enabled: bool,
    _read_only: bool,
    sandbox_unavailable: bool,
    env_allows_direct: bool,
    approved_direct: bool,
) -> bool {
    sandbox_enabled && !(sandbox_unavailable && env_allows_direct && approved_direct)
}