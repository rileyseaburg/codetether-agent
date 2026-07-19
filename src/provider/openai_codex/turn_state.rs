//! Turn-scoped sticky-routing state shared across retry attempts.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};

const MAX_TURNS: usize = 4_096;
type State = Arc<OnceLock<String>>;

#[derive(Clone, Debug, Default)]
pub(super) struct TurnStateStore(Arc<Mutex<HashMap<String, State>>>);

impl TurnStateStore {
    pub(super) fn begin(&self, session_id: &str) {
        let mut states = self.states();
        if states.len() >= MAX_TURNS && !states.contains_key(session_id) {
            states.clear();
        }
        states.insert(session_id.to_string(), Arc::new(OnceLock::new()));
    }

    pub(super) fn current(&self, session_id: &str) -> State {
        self.states()
            .entry(session_id.to_string())
            .or_insert_with(|| Arc::new(OnceLock::new()))
            .clone()
    }

    pub(super) fn capture(&self, session_id: &str, value: &str) {
        let _ = self.current(session_id).set(value.to_string());
    }

    fn states(&self) -> MutexGuard<'_, HashMap<String, State>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

#[cfg(test)]
#[path = "turn_state_tests.rs"]
mod tests;
