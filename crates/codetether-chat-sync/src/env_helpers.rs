//! Environment variable helpers for chat sync configuration.

/// Read a trimmed nonempty environment value.
/// # Arguments
/// Environment variable name.
/// # Returns
/// A nonempty trimmed value when present.
/// # Examples
/// ```text
/// Configure the archive destination, then use env_non_empty through the chat-sync API.
/// ```
pub fn env_non_empty(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Parse common boolean environment values.
/// # Arguments
/// Environment variable name.
/// # Returns
/// Parsed boolean, or None for an absent or invalid value.
/// # Examples
/// ```text
/// Configure the archive destination, then use env_bool through the chat-sync API.
/// ```
pub fn env_bool(name: &str) -> Option<bool> {
    let value = env_non_empty(name)?;
    match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

/// Recognize conventional credential placeholders.
/// # Arguments
/// Candidate credential text.
/// # Returns
/// Whether the text is a known placeholder.
/// # Examples
/// ```text
/// Configure the archive destination, then use is_placeholder_secret through the chat-sync API.
/// ```
pub fn is_placeholder_secret(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "replace-me" | "changeme" | "change-me" | "your-token" | "your-key"
    )
}

/// Read an environment value excluding placeholder credentials.
/// # Arguments
/// Environment variable name.
/// # Returns
/// A nonempty non-placeholder value when present.
/// # Examples
/// ```text
/// Configure the archive destination, then use env_non_placeholder through the chat-sync API.
/// ```
pub fn env_non_placeholder(name: &str) -> Option<String> {
    env_non_empty(name).filter(|v| !is_placeholder_secret(v))
}
