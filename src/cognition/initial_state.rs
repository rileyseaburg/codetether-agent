//! Preserve empty test initialization while sharing production restoration.
pub(super) use codetether_cognition_core::runtime::InitialState;
#[cfg(test)]
pub(super) fn load_initial_state() -> InitialState {
    InitialState::default()
}
#[cfg(not(test))]
pub(super) use codetether_cognition_storage::load_initial_state;
