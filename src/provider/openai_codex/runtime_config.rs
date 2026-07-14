//! Thread-safe OpenAI Codex reasoning-effort override.

use parking_lot::RwLock;
use std::sync::OnceLock;

fn cell() -> &'static RwLock<Option<String>> {
    static CELL: OnceLock<RwLock<Option<String>>> = OnceLock::new();
    CELL.get_or_init(|| RwLock::new(seed_from_env()))
}

fn seed_from_env() -> Option<String> {
    std::env::var("CODETETHER_OPENAI_CODEX_THINKING_LEVEL")
        .ok()
        .or_else(|| std::env::var("CODETETHER_OPENAI_CODEX_REASONING_EFFORT").ok())
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
}

/// Return the current normalized reasoning-effort override.
///
/// # Returns
///
/// Returns `None` when no process-wide override is configured.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::openai_codex::runtime_config::thinking_level;
/// let _current_override = thinking_level();
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
/// use codetether_agent::provider::openai_codex::runtime_config::{
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
mod tests {
    use super::{set_thinking_level, thinking_level};

    #[test]
    fn override_round_trips_without_environment_mutation() {
        let previous = thinking_level();
        set_thinking_level(Some("xhigh".to_string()));
        assert_eq!(thinking_level().as_deref(), Some("xhigh"));
        set_thinking_level(previous);
    }
}
