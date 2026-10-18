//! Debounced scheduling from picker state.
use codetether_lsp_tool::Host;
use codetether_symbol_search::SymbolSearchState;

/// Schedule a cancellable, trailing-debounced symbol refresh.
/// # Arguments
/// * `state` - Picker containing the current query and cached language files.
/// # Returns
/// Unit after scheduling; short queries clear results without contacting LSP.
/// # Panics
/// Panics if called without a Tokio runtime for a searchable query, or if a
/// process-wide task mutex is poisoned.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_lsp_tool::Host>() {
/// let mut state = codetether_symbol_search::SymbolSearchState::new();
/// codetether_symbol_runtime::schedule::<H>(&mut state);
/// # }
/// ```
pub fn schedule<H: Host>(state: &mut SymbolSearchState) {
    let query = state.query.clone();
    if !super::gate::is_searchable(&query) {
        super::runtime::cancel();
        state.set_results(Vec::new());
        state.loading = false;
        return;
    }
    state.loading = true;
    let root = match std::env::current_dir() {
        Ok(root) => root,
        Err(error) => {
            state.set_error(error.to_string());
            return;
        }
    };
    let files = state.language_files().to_vec();
    super::runtime::schedule::<H>(query, files, root);
}
