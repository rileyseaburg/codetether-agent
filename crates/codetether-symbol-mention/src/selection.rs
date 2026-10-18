//! Confirm a symbol from mention mode or the standalone search picker.
use crate::Composer;

/// Confirm a mention or finish standalone symbol selection.
///
/// # Arguments
/// * `app` — Composer and current symbol picker.
/// # Returns
/// Updates status and closes standalone search; mention handling keeps its
/// existing behavior when no symbol is selected.
/// # Panics
/// Panics if the shared symbol runtime's task mutex is poisoned.
/// # Examples
/// ```no_run
/// # fn select(app: &mut impl codetether_symbol_mention::Composer) {
/// codetether_symbol_mention::confirm_selection(app);
/// # }
/// ```
pub fn confirm_selection(app: &mut impl Composer) {
    if crate::confirm(app) {
        return;
    }
    if let Some(symbol) = app.picker().selected_symbol().cloned() {
        let location = symbol
            .line
            .map(|line| format!("at line {line}"))
            .unwrap_or_default();
        app.set_status(format!("Selected symbol {} {location}", symbol.name));
    }
    codetether_symbol_runtime::cancel();
    app.picker_mut().close();
}
