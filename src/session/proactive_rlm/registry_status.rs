//! Registry generation and lifecycle queries.

use super::registry::STATES;

pub(super) fn is_current(session_id: &str, generation: u64) -> bool {
    STATES
        .get_or_init(Default::default)
        .lock()
        .ok()
        .and_then(|states| {
            states
                .get(session_id)
                .map(|state| state.slot.is_current(generation))
        })
        .unwrap_or(false)
}

pub(super) fn exists(session_id: &str) -> bool {
    STATES
        .get_or_init(Default::default)
        .lock()
        .is_ok_and(|states| states.contains_key(session_id))
}

pub(super) fn remove(session_id: &str) {
    if let Ok(mut states) = STATES.get_or_init(Default::default).lock() {
        states.remove(session_id);
    }
}
