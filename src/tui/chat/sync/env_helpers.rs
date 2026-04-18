//! Environment variable helpers for chat sync configuration.

pub fn env_non_empty(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn env_bool(name: &str) -> Option<bool> {
    let value = env_non_empty(name)?;
    match value.to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

pub fn is_placeholder_secret(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "replace-me" | "changeme" | "change-me" | "your-token" | "your-key"
    )
}

pub fn env_non_placeholder(name: &str) -> Option<String> {
    env_non_empty(name).filter(|v| !is_placeholder_secret(v))
}
