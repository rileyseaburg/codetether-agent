//! Turn-scoped sticky-routing state shared across retry attempts.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};

const MAX_TURNS: usize = 4_096;
type State = Arc<OnceLock<String>>;

/// First-server-value routing state, reset when a new turn begins.
///
/// # Examples
/// ```
/// let states = codetether_codex_transport::TurnStateStore::default();
/// states.begin("session");
/// states.capture("session", "sticky");
/// assert_eq!(states.current("session").get().map(String::as_str), Some("sticky"));
/// ```
#[derive(Clone, Debug, Default)]
pub struct TurnStateStore(Arc<Mutex<HashMap<String, State>>>);

impl TurnStateStore {
    /// Replace the session routing state for a new turn.
    pub fn begin(&self, session_id: &str) {
        let mut states = self.states();
        if states.len() >= MAX_TURNS && !states.contains_key(session_id) {
            states.clear();
        }
        states.insert(session_id.to_string(), Arc::new(OnceLock::new()));
    }

    /// Return shared routing state, creating an empty slot when absent.
    pub fn current(&self, session_id: &str) -> State {
        self.states()
            .entry(session_id.to_string())
            .or_insert_with(|| Arc::new(OnceLock::new()))
            .clone()
    }

    /// Record the first routing value observed for this turn.
    pub fn capture(&self, session_id: &str, value: &str) {
        let _ = self.current(session_id).set(value.to_string());
    }

    fn states(&self) -> MutexGuard<'_, HashMap<String, State>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
#[path = "turn_state_tests.rs"]
mod tests;
