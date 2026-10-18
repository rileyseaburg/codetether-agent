//! End a worker only after its pending slot is empty.
use super::state::states;

pub(super) fn settle(session_id: &str) -> bool {
    let Ok(mut states) = states().lock() else {
        return true;
    };
    let Some(state) = states.get_mut(session_id) else {
        return true;
    };
    if state.pending.is_some() {
        return false;
    }
    state.running = false;
    true
}
