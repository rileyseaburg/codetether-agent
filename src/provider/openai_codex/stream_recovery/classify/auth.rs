//! WebSocket handshake status classification.

pub(in crate::provider::openai_codex) fn is_upgrade_required(error: &anyhow::Error) -> bool {
    let message = format!("{error:#}").to_ascii_lowercase();
    message.contains("426") || message.contains("upgrade required")
}

pub(in crate::provider::openai_codex) fn is_unauthorized(error: &anyhow::Error) -> bool {
    let message = format!("{error:#}").to_ascii_lowercase();
    message.contains("401") || message.contains("unauthorized")
}
