//! Initial-state policy kept explicit across native and test callers.
use codetether_cognition_core::runtime::InitialState;

pub(super) fn load<const EMPTY_STATE: bool>() -> InitialState {
    if EMPTY_STATE {
        InitialState::default()
    } else {
        codetether_cognition_storage::load_initial_state()
    }
}
