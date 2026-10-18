//! Replace the mention marker with the selected symbol.
use crate::Composer;
use codetether_symbol_search::SymbolSearchMode;

/// Replace the recorded at-sign with the selected symbol's mention.
/// # Arguments
/// * `app` - Host composer with the selection to attach.
/// # Returns
/// False outside mention mode; true in mention mode, including no selection.
/// # Panics
/// Panics if the shared symbol runtime's task mutex is poisoned.
/// # Examples
/// ```no_run
/// # fn example(app: &mut impl codetether_symbol_mention::Composer) {
/// let handled = codetether_symbol_mention::confirm(app);
/// # }
/// ```
pub fn confirm(app: &mut impl Composer) -> bool {
    let SymbolSearchMode::Mention { start } = app.picker().mode else {
        return false;
    };
    let Some(symbol) = app.picker().selected_symbol().cloned() else {
        return true;
    };
    let mention = symbol.mention();
    app.replace_input_chars(start..start + 1, &mention);
    codetether_symbol_runtime::cancel();
    app.picker_mut().close();
    app.set_status(format!("Attached symbol {}", symbol.name));
    true
}
