//! Process-wide "prompts become goals" toggle.
//!
//! When enabled, a plain chat prompt submitted while the session has no
//! unfinished goal is adopted as the session goal, so the turn keeps
//! continuing until the independent verifier accepts `update_goal complete`.
//! The initial value comes from `CODETETHER_AUTO_GOAL` (`1`/`true`/`on`).

use std::sync::atomic::{AtomicU8, Ordering};

const UNSET: u8 = 0;
const OFF: u8 = 1;
const ON: u8 = 2;

static MODE: AtomicU8 = AtomicU8::new(UNSET);

/// Return whether submitted prompts are adopted as session goals.
pub(crate) fn enabled() -> bool {
    match MODE.load(Ordering::Relaxed) {
        ON => true,
        OFF => false,
        _ => from_env(),
    }
}

/// Enable or disable prompt-to-goal adoption for this process.
pub(crate) fn set_enabled(on: bool) {
    MODE.store(if on { ON } else { OFF }, Ordering::Relaxed);
}

fn from_env() -> bool {
    std::env::var("CODETETHER_AUTO_GOAL").is_ok_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "on" | "yes"
        )
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn explicit_setting_overrides_env() {
        super::set_enabled(true);
        assert!(super::enabled());
        super::set_enabled(false);
        assert!(!super::enabled());
    }
}
