use super::openrouter_thinking_effort_label;
use crate::provider::openrouter::runtime_config;
use std::sync::{Mutex, MutexGuard, OnceLock};

/// Serializes label assertions against the process-wide effort override.
fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[test]
fn label_reports_default_when_no_override_is_set() {
    let _lock = guard();
    let previous = runtime_config::thinking_level();
    runtime_config::set_thinking_level(None);
    assert_eq!(openrouter_thinking_effort_label(), "default");
    runtime_config::set_thinking_level(previous);
}

#[test]
fn label_reflects_the_active_override() {
    let _lock = guard();
    let previous = runtime_config::thinking_level();
    runtime_config::set_thinking_level(Some("xhigh".to_string()));
    assert_eq!(openrouter_thinking_effort_label(), "xhigh");
    runtime_config::set_thinking_level(previous);
}

#[test]
fn cycle_order_matches_the_wire_level_order() {
    // Keeps the Settings panel hint honest about the cycle sequence.
    let levels = crate::provider::openrouter::reasoning_levels::LEVELS;
    assert_eq!(levels.first().copied(), Some("none"));
    assert_eq!(levels.last().copied(), Some("max"));
}
