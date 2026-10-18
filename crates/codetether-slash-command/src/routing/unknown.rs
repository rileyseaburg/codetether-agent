//! Existing post-dispatch unknown-message suppression list.
/// Report whether a normalized command suppresses the unknown-command notice.
/// # Arguments
/// * `normalized` - Full normalized command string.
/// # Returns
/// True only for names in the existing fallback suppression list.
/// # Examples
/// ~~~
/// assert!(codetether_slash_command::routing::suppress_unknown("/help"));
/// ~~~
pub fn suppress_unknown(normalized: &str) -> bool {
    matches!(
        normalized,
        "/help"
            | "/sessions"
            | "/import-codex"
            | "/session"
            | "/swarm"
            | "/ralph"
            | "/bus"
            | "/protocol"
            | "/model"
            | "/settings"
            | "/lsp"
            | "/rlm"
            | "/latency"
            | "/transport"
            | "/audit"
            | "/git"
            | "/auditloop"
            | "/chat"
            | "/home"
            | "/main"
            | "/symbols"
            | "/symbol"
            | "/new"
            | "/undo"
            | "/keys"
            | "/file"
            | "/image"
            | "/autoapply"
            | "/network"
            | "/autocomplete"
            | "/mcp"
            | "/context"
            | "/spawn"
            | "/kill"
            | "/agents"
            | "/agent"
            | "/autochat"
            | "/protocols"
            | "/registry"
            | "/recall"
    )
}
