//! Install a file snippet in the interactive composer.
use super::View;
pub(super) fn append<V: View>(view: &mut V, snippet: &str) {
    let input = view.input();
    if !input.trim().is_empty() {
        input.push_str("\n\n");
    }
    input.push_str(snippet);
    let cursor = input.chars().count();
    view.set_cursor(cursor);
    view.start_editing();
    view.clear_history_selection();
    view.refresh_suggestions();
    view.show_chat();
    view.scroll_to_bottom();
}
