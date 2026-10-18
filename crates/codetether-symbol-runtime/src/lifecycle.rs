//! Picker activation and cancellation boundary.
use codetether_symbol_search::SymbolSearchState;

/// Reports whether the picker is open.
/// # Arguments
/// * `state` - Picker to inspect.
/// # Returns
/// Its current active flag.
/// # Examples
/// ```
/// let state = codetether_symbol_search::SymbolSearchState::new();
/// assert!(!codetether_symbol_runtime::active(&state));
/// ```
pub fn active(state: &SymbolSearchState) -> bool {
    state.active
}

/// Close the picker and cancel its pending background query.
/// # Arguments
/// * `state` - Picker whose cached UI state will be cleared.
/// # Returns
/// Unit after cancellation and reset.
/// # Panics
/// Panics if the process-wide task mutex is poisoned.
/// # Examples
/// ```
/// let mut state = codetether_symbol_search::SymbolSearchState::new();
/// codetether_symbol_runtime::close(&mut state);
/// assert!(!state.active);
/// ```
pub fn close(state: &mut SymbolSearchState) {
    super::runtime::cancel();
    state.close();
}
