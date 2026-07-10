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
pub fn thinking_level() -> Option<String> {
    cell().read().clone()
}

/// Replace the process-wide reasoning-effort override.
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
