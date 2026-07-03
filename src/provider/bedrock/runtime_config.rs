//! Thread-safe runtime overrides for Bedrock request fields.
//!
//! Replaces runtime `std::env::set_var` mutation (unsafe in multi-threaded
//! async contexts) with a process-wide `RwLock`. Values are seeded once from
//! the corresponding env vars so externally-configured deployments keep
//! working; the TUI Settings panel updates them in-process at runtime.

use parking_lot::RwLock;
use std::sync::OnceLock;

#[derive(Default, Clone)]
struct BedrockOverrides {
    thinking_effort: Option<String>,
    service_tier: Option<String>,
}

fn cell() -> &'static RwLock<BedrockOverrides> {
    static CELL: OnceLock<RwLock<BedrockOverrides>> = OnceLock::new();
    CELL.get_or_init(|| RwLock::new(seed_from_env()))
}

fn seed_from_env() -> BedrockOverrides {
    BedrockOverrides {
        thinking_effort: env_value("CODETETHER_BEDROCK_THINKING_EFFORT"),
        service_tier: env_value("CODETETHER_BEDROCK_SERVICE_TIER"),
    }
}

fn env_value(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|v| v.trim().to_ascii_lowercase())
        .filter(|v| !v.is_empty())
}

/// Current thinking-effort override (normalized), if any.
pub fn thinking_effort() -> Option<String> {
    cell().read().thinking_effort.clone()
}

/// Set (or clear with `None`) the thinking-effort override.
pub fn set_thinking_effort(value: Option<String>) {
    cell().write().thinking_effort = value;
}

/// Current service-tier override (normalized), if any.
pub fn service_tier() -> Option<String> {
    cell().read().service_tier.clone()
}

/// Set (or clear with `None`) the service-tier override.
pub fn set_service_tier(value: Option<String>) {
    cell().write().service_tier = value;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overrides_round_trip_without_env_mutation() {
        set_thinking_effort(Some("high".to_string()));
        assert_eq!(thinking_effort().as_deref(), Some("high"));
        set_service_tier(None);
        assert_eq!(service_tier(), None);
    }
}