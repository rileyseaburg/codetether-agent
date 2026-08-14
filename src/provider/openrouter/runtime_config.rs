//! Thread-safe OpenRouter reasoning-effort override.
//!
//! Mirrors the Codex override so the TUI can change thinking level in-process
//! without `std::env::set_var`, which is unsafe in multi-threaded async code.

use parking_lot::RwLock;
use std::sync::OnceLock;

fn cell() -> &'static RwLock<Option<String>> {
    static CELL: OnceLock<RwLock<Option<String>>> = OnceLock::new();
    CELL.get_or_init(|| RwLock::new(seed_from_env()))
}

fn seed_from_env() -> Option<String> {
    std::env::var("CODETETHER_OPENROUTER_THINKING_LEVEL")
        .ok()
        .or_else(|| std::env::var("CODETETHER_OPENROUTER_REASONING_EFFORT").ok())
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
}

/// Current normalized reasoning-effort override, if any.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::openrouter::runtime_config::thinking_level;
/// let _current = thinking_level();
/// ```
pub fn thinking_level() -> Option<String> {
    cell().read().clone()
}

/// Replace the process-wide reasoning-effort override.
///
/// # Arguments
///
/// * `value` — New wire-level effort, or `None` to clear the override.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::provider::openrouter::runtime_config::{
///     set_thinking_level, thinking_level,
/// };
/// let previous = thinking_level();
/// set_thinking_level(Some("high".to_string()));
/// assert_eq!(thinking_level().as_deref(), Some("high"));
/// set_thinking_level(previous);
/// ```
pub fn set_thinking_level(value: Option<String>) {
    *cell().write() = value;
}

#[cfg(test)]
#[path = "runtime_config_tests.rs"]
mod tests;
