//! Preserve typed text when dismissing mention selection.
use crate::Composer;

/// Abandon an open mention, keeping the text the user already typed.
///
/// The `@` and query stay in the chat buffer, so a stray `@` never costs the
/// user their input.
/// # Arguments
/// * `app` - Host composer containing the active picker query.
/// # Returns
/// Unit after cancellation, insertion of the typed query and status clearing.
/// # Panics
/// Panics if the shared symbol runtime's task mutex is poisoned.
/// # Examples
/// ```no_run
/// # fn example(app: &mut impl codetether_symbol_mention::Composer) {
/// codetether_symbol_mention::dismiss(app);
/// # }
/// ```
pub fn dismiss(app: &mut impl Composer) {
    let typed = app.picker().query.clone();
    codetether_symbol_runtime::cancel();
    app.picker_mut().close();
    app.insert_text(&typed);
    app.set_status(String::new());
}
