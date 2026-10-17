//! Load a bounded state file and decode its schema.
use super::{LoadEvent, PersistedCognitionState};
use std::path::Path;

/// Load compatible persisted state from an explicit path.
///
/// # Arguments
/// * `path` - File to inspect and read.
/// * `report` - Receives diagnostics at each original logging point.
///
/// # Returns
/// State when valid, otherwise None. Oversized files are renamed with a bloated suffix.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_core::persistence::load_state;
/// let state = load_state(std::path::Path::new(".codetether/cognition/state.json"), |_| {});
/// if let Some(state) = state { assert_eq!(state.schema_version, 1); }
/// ```
pub fn load_state(
    path: &Path,
    mut report: impl FnMut(LoadEvent<'_>),
) -> Option<PersistedCognitionState> {
    if !super::guard::allow(path, &mut report) {
        return None;
    }
    let data = match std::fs::read_to_string(path) {
        Ok(d) => d,
        Err(e) => {
            report(LoadEvent::ReadFailed(&e));
            return None;
        }
    };

    super::decode::decode(&data, &mut report)
}
