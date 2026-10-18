//! Supply the application path and logger when restoring cognition state.
use super::{PersistedCognitionState, log_load, path::state_path};
/// Load the saved cognition state from the application's data directory.
/// # Returns
/// The validated snapshot, or `None` when it is missing or rejected by the
/// shared loader. Diagnostic events retain their original tracing target.
/// # Examples
/// ```no_run
/// let state = codetether_cognition_storage::load_state();
/// if let Some(state) = state {
///     assert!(!state.persisted_at.to_string().is_empty());
/// }
/// ```
pub fn load_state() -> Option<PersistedCognitionState> {
    codetether_cognition_core::persistence::load_state(&state_path(), log_load)
}
