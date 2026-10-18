//! Drain background forage updates into the chat view.

use super::View;

use super::drain_map::map_update;
use super::state::ForageUpdate;

/// Drain pending forage updates into chat messages on the background tick.
///
/// Returns `true` when at least one update was applied so the caller can
/// gate a redraw.
///
/// # Arguments
/// * `app` — View receiving messages, status changes and scroll updates.
///
/// # Returns
/// Whether pending updates were applied.
///
/// # Examples
/// ```no_run
/// # fn tick(view: &mut impl codetether_forage_interactive::View) {
/// let changed = codetether_forage_interactive::drain_forage_updates(view);
/// # let _ = changed;
/// # }
/// ```
pub fn drain_forage_updates(app: &mut impl View) -> bool {
    let updates: Vec<ForageUpdate> = app
        .state()
        .rx
        .as_mut()
        .map(|rx| {
            let mut out = Vec::new();
            while let Ok(u) = rx.try_recv() {
                out.push(u);
            }
            out
        })
        .unwrap_or_default();
    if updates.is_empty() {
        return false;
    }
    for update in updates {
        apply_forage_update(app, update);
    }
    true
}

fn apply_forage_update(app: &mut impl View, update: ForageUpdate) {
    let (text, status) = map_update(app.state(), update);
    app.push_system(text);
    app.set_status(status);
    app.scroll_to_bottom();
}
