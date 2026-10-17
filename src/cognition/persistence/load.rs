//! Supply the application path and logger when restoring cognition state.
use super::{PersistedCognitionState, log_load, path::state_path};
pub fn load_state() -> Option<PersistedCognitionState> {
    codetether_cognition_core::persistence::load_state(&state_path(), log_load)
}
