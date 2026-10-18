//! Process-wide one-shot prompt retained until a host can submit it.
//! A new stored prompt replaces the previous one; taking empties the slot.
use std::sync::{LazyLock, Mutex};

static QUEUED: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
/// Replace the queued prompt with an already validated follow-up.
///
/// # Arguments
/// * `prompt` — Host-validated prompt to retain.
///
/// # Returns
/// Stores the prompt without changing any host UI state.
///
/// # Panics
/// Panics if the queue mutex was poisoned.
///
/// # Examples
/// ```no_run
/// codetether_session_steering::follow_up::store("continue".into());
/// ```
pub fn store(prompt: String) {
    *QUEUED.lock().expect("queued prompt mutex poisoned") = Some(prompt);
}

/// Remove and return the queued follow-up.
///
/// # Returns
/// The stored prompt, or None when no prompt is queued.
///
/// # Panics
/// Panics if the queue mutex was poisoned.
///
/// # Examples
/// ```no_run
/// let _prompt = codetether_session_steering::follow_up::take();
/// ```
pub fn take() -> Option<String> {
    QUEUED.lock().expect("queued prompt mutex poisoned").take()
}

/// Check whether a follow-up is queued.
///
/// # Returns
/// Whether the shared slot currently contains a prompt.
///
/// # Panics
/// Panics if the queue mutex was poisoned.
///
/// # Examples
/// ```no_run
/// let _pending = codetether_session_steering::follow_up::has_pending();
/// ```
pub fn has_pending() -> bool {
    QUEUED
        .lock()
        .expect("queued prompt mutex poisoned")
        .is_some()
}
