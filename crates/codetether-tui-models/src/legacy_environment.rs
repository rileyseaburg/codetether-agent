//! Environment classification retained by the legacy TUI helper API.

/// Classify the environment using explicit flags before its deployment name.
/// # Arguments
/// Receives the primary flag, legacy flag, and optional environment name.
/// # Returns
/// The first explicit flag, or whether the normalized name is secure/production.
/// # Examples
/// See the public [`is_secure_environment`] compatibility entry point.
pub(crate) fn is_secure_environment_from_values(
    secure_environment: Option<bool>,
    secure_env: Option<bool>,
    environment_name: Option<&str>,
) -> bool {
    if let Some(value) = secure_environment {
        return value;
    }
    if let Some(value) = secure_env {
        return value;
    }
    environment_name.is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "secure" | "production" | "prod"
        )
    })
}

/// Classify the deployment using the original TUI environment variables.
/// # Arguments
/// Reads `CODETETHER_SECURE_ENVIRONMENT`, `CODETETHER_SECURE_ENV`, and `CODETETHER_ENV`.
/// # Returns
/// The first valid explicit boolean, falling back to the deployment name.
/// # Examples
/// ```no_run
/// let secure = codetether_tui_models::legacy_helpers::is_secure_environment();
/// # let _ = secure;
/// ```
pub fn is_secure_environment() -> bool {
    is_secure_environment_from_values(
        env_bool("CODETETHER_SECURE_ENVIRONMENT"),
        env_bool("CODETETHER_SECURE_ENV"),
        env_non_empty("CODETETHER_ENV").as_deref(),
    )
}

fn env_non_empty(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn env_bool(name: &str) -> Option<bool> {
    let value = env_non_empty(name)?;
    let value = value.to_ascii_lowercase();
    match value.as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}
