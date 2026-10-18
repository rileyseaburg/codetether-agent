//! Apply current results and discard stale queries.
use crate::active;
use codetether_symbol_search::SymbolSearchState;

/// Apply the newest completed search when it still matches the UI query.
/// # Arguments
/// * `state` - Picker whose active flag and query gate acceptance.
/// # Returns
/// Whether a completed result was accepted; errors update the picker too.
/// # Panics
/// Panics if the completion-slot mutex is poisoned.
/// # Examples
/// ```
/// let mut state = codetether_symbol_search::SymbolSearchState::new();
/// assert!(!codetether_symbol_runtime::drain(&mut state));
/// ```
pub fn drain(state: &mut SymbolSearchState) -> bool {
    let Some(done) = super::runtime::take() else {
        return false;
    };
    if !active(state) || state.query != done.query {
        return false;
    }
    state.set_language_files(done.files);
    match done.result {
        Ok(results) => state.set_results(results),
        Err(error) => state.set_error(error.to_string()),
    }
    true
}
