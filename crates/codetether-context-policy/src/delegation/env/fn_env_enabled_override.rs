//! env enabled override policy.
use super::*;

/// Read `CODETETHER_DELEGATION_ENABLED` env var override.
pub fn env_enabled_override() -> Option<bool> {
    let raw = env::var("CODETETHER_DELEGATION_ENABLED").ok()?;
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}
