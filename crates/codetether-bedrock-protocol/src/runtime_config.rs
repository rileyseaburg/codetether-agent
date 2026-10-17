//! Thread-safe runtime overrides for Bedrock request fields.
//!
//! Replaces runtime `std::env::set_var` mutation (unsafe in multi-threaded
//! async contexts) with a process-wide `RwLock`. Values are seeded once from
//! the corresponding env vars so externally-configured deployments keep
//! working; the TUI Settings panel updates them in-process at runtime.

//!
//! Read overrides with [thinking_effort] and [service_tier], and update them
//! through [set_thinking_effort] and [set_service_tier]. All callers share one
//! cell, including application compatibility exports.

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
///
/// # Arguments
/// This getter takes no arguments.
///
/// # Returns
/// The current optional override, seeding the shared cell on first access.
///
/// # Examples
/// ```no_run
/// let current = codetether_bedrock_protocol::runtime_config::thinking_effort();
/// assert_eq!(current, codetether_bedrock_protocol::runtime_config::thinking_effort());
/// ```
pub fn thinking_effort() -> Option<String> {
    cell().read().thinking_effort.clone()
}

/// Set (or clear with `None`) the thinking-effort override.
///
/// # Arguments
/// * `value` - Replacement override, or `None` to clear it.
///
/// # Returns
/// Updates the process-wide override in place.
///
/// # Examples
/// ```no_run
/// use codetether_bedrock_protocol::runtime_config;
/// let previous = runtime_config::thinking_effort();
/// runtime_config::set_thinking_effort(None);
/// assert_eq!(runtime_config::thinking_effort(), None);
/// runtime_config::set_thinking_effort(previous);
/// ```
pub fn set_thinking_effort(value: Option<String>) {
    cell().write().thinking_effort = value;
}

/// Current service-tier override (normalized), if any.
///
/// # Arguments
/// This getter takes no arguments.
///
/// # Returns
/// The current optional override, seeding the shared cell on first access.
///
/// # Examples
/// ```no_run
/// let current = codetether_bedrock_protocol::runtime_config::service_tier();
/// assert_eq!(current, codetether_bedrock_protocol::runtime_config::service_tier());
/// ```
pub fn service_tier() -> Option<String> {
    cell().read().service_tier.clone()
}

/// Set (or clear with `None`) the service-tier override.
///
/// # Arguments
/// * `value` - Replacement override, or `None` to clear it.
///
/// # Returns
/// Updates the process-wide override in place.
///
/// # Examples
/// ```no_run
/// use codetether_bedrock_protocol::runtime_config;
/// let previous = runtime_config::service_tier();
/// runtime_config::set_service_tier(None);
/// assert_eq!(runtime_config::service_tier(), None);
/// runtime_config::set_service_tier(previous);
/// ```
pub fn set_service_tier(value: Option<String>) {
    cell().write().service_tier = value;
}
#[cfg(test)]
#[path = "runtime_config/tests.rs"]
mod tests;
