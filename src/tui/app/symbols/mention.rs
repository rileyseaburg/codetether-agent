//! Application composer binding for shared symbol mention behavior.
mod host;
use crate::tui::app::state::App;
pub use codetether_symbol_mention::starts_mention;

/// Open mention selection using the native composer's editing behavior.
/// # Arguments
/// * `app` — Current editor, symbol picker and status.
/// # Returns
/// Whether the picker opened; otherwise inserts a literal at-sign.
/// # Examples
/// ```
/// let mut app = codetether_agent::tui::app::state::App::default();
/// assert!(codetether_agent::tui::app::symbols::mention::open(&mut app));
/// ```
pub fn open(app: &mut App) -> bool {
    codetether_symbol_mention::open(&mut host::view(app))
}

/// Confirm the current mention using the native composer's editing behavior.
/// # Arguments
/// * `app` — Current editor, symbol picker and status.
/// # Returns
/// Whether the picker is in mention mode.
/// # Panics
/// Panics if the shared symbol runtime's task mutex is poisoned.
/// # Examples
/// ```
/// let mut app = codetether_agent::tui::app::state::App::default();
/// assert!(!codetether_agent::tui::app::symbols::mention::confirm(&mut app));
/// ```
pub fn confirm(app: &mut App) -> bool {
    codetether_symbol_mention::confirm(&mut host::view(app))
}

/// Dismiss mention selection, preserving the typed query in the composer.
/// # Arguments
/// * `app` — Current editor, symbol picker and status.
/// # Returns
/// Closes the picker, inserts its query and clears status.
/// # Panics
/// Panics if the shared symbol runtime's task mutex is poisoned.
/// # Examples
/// ```no_run
/// let mut app = codetether_agent::tui::app::state::App::default();
/// codetether_agent::tui::app::symbols::mention::dismiss(&mut app);
/// ```
pub fn dismiss(app: &mut App) {
    codetether_symbol_mention::dismiss(&mut host::view(app));
}

/// Confirm a mention or the standalone symbol-picker selection.
/// # Arguments
/// * `app` — Current editor, symbol picker and status.
/// # Returns
/// Applies mention replacement or closes standalone selection.
/// # Panics
/// Panics if the shared symbol runtime's task mutex is poisoned.
/// # Examples
/// ```no_run
/// let mut app = codetether_agent::tui::app::state::App::default();
/// codetether_agent::tui::app::symbols::mention::confirm_selection(&mut app);
/// ```
pub fn confirm_selection(app: &mut App) {
    codetether_symbol_mention::confirm_selection(&mut host::view(app));
}
#[cfg(test)]
mod tests;
