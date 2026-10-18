//! Finder keyboard dispatch and redraw policy.
use super::FinderView;
use codetether_fuzzy_finder::FinderAction;
use crossterm::event::KeyEvent;
use std::path::Path;

/// Handle a key press while the fuzzy finder is open.
///
/// Returns `true` if the key was consumed by the finder.
/// # Arguments
/// * `view` — Finder state, editor opening and redraw operations.
/// * `cwd` — Workspace root for the selected file's status label.
/// * `key` — Incoming terminal key.
/// # Returns
/// True when the finder is open and consumes the key, otherwise false.
/// # Examples
/// ```no_run
/// # fn example(view: &mut impl codetether_editor::interactive::FinderView, key: crossterm::event::KeyEvent) {
/// let _ = codetether_editor::interactive::handle_finder_key(view, std::path::Path::new("."), key);
/// # }
/// ```
pub fn handle_finder_key(view: &mut impl FinderView, cwd: &Path, key: KeyEvent) -> bool {
    let Some(finder) = view.finder().as_mut() else {
        return false;
    };
    match finder.handle_key(key) {
        FinderAction::Cancel => {
            *view.finder() = None;
            view.set_status("Fuzzy open cancelled".to_string());
        }
        FinderAction::Confirm => super::confirm_finder(view, cwd),
        FinderAction::Continue => {}
    }
    view.redraw();
    true
}
