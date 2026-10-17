//! WebSocket handshake status classification.

/// Detect a handshake failure that permits HTTP fallback.
///
/// # Arguments
/// * `error` - Full contextual transport failure.
/// # Returns
/// True for the preserved forbidden/upgrade markers.
/// # Examples
/// ```
/// assert!(codetether_codex_transport::is_upgrade_required(&anyhow::anyhow!("HTTP error: 403 Forbidden")));
/// ```
pub fn is_upgrade_required(error: &anyhow::Error) -> bool {
    let message = format!("{error:#}").to_ascii_lowercase();
    message.contains("403 forbidden")
        || message.contains("426")
        || message.contains("upgrade required")
}

/// Detect an unauthorized response using its full error chain text.
///
/// # Arguments
/// * `error` - Contextual transport failure.
/// # Returns
/// Whether the message includes a 401 or unauthorized marker.
/// # Examples
/// ```
/// assert!(codetether_codex_transport::is_unauthorized(&anyhow::anyhow!("401 Unauthorized")));
/// ```
pub fn is_unauthorized(error: &anyhow::Error) -> bool {
    let message = format!("{error:#}").to_ascii_lowercase();
    message.contains("401") || message.contains("unauthorized")
}

#[cfg(test)]
#[path = "auth_tests.rs"]
mod tests;
