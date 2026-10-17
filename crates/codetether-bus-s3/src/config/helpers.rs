//! Credential field lookup helpers.
use super::*;

/// Read an env var, returning `None` if unset or empty.
pub(super) fn env_non_empty(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|s| !s.is_empty())
}

/// Extract a string value from `ProviderSecrets.extra`, trying multiple key names.
pub(super) fn vault_extra_str(secrets: &secrets::ProviderSecrets, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(val) = secrets.extra.get(*key)
            && let Some(s) = val.as_str()
            && !s.is_empty()
        {
            return Some(s.to_string());
        }
    }
    None
}
