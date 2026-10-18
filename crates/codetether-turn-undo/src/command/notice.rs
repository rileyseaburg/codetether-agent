//! Final undo notice including partial-history availability.
use super::View;
pub(super) fn show<V: View>(view: &mut V, undo_count: usize, n: usize) {
    let partial_note = if undo_count < n {
        format!(" (only {undo_count} available)")
    } else {
        String::new()
    };
    view.push_system_message(format!("Undid {undo_count} turn(s){partial_note}."));
}
