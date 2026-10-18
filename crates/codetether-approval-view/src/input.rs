//! Keyboard interaction for pending approval previews.
use codetether_approval_queue as approval_queue;
use crossterm::event::{KeyCode, KeyEvent};
/// Borrowed preview state needed for navigation and scrolling.
/// # Examples
/// ```no_run
/// let mut scroll = 0;
/// let state = codetether_approval_view::input::Preview {
///     chat: true, editing: false, help: false, input: "", scroll: &mut scroll };
/// ```
pub struct Preview<'a> {
    /// Whether the active view is chat.
    pub chat: bool,
    /// Whether an approval editor is open.
    pub editing: bool,
    /// Whether help obscures the preview.
    pub help: bool,
    /// Composer input used to exclude approval feedback entry.
    pub input: &'a str,
    /// Current preview scroll position.
    pub scroll: &'a mut usize,
}
/// Apply queue navigation or bounded scrolling to an active preview.
/// # Arguments
/// Receives the current preview state and key event.
/// # Returns
/// Whether the preview consumed the key.
/// # Examples
/// ```no_run
/// let mut scroll = 0;
/// let state = codetether_approval_view::input::Preview { chat: true, editing: false, help: false, input: "", scroll: &mut scroll };
/// let _ = codetether_approval_view::input::handle(state, crossterm::event::KeyCode::Down.into());
/// ```
pub fn handle(state: Preview<'_>, key: KeyEvent) -> bool {
    if !state.chat || state.editing || state.help || approval_queue::feedback_input(state.input) {
        return false;
    }
    let Some(item) = approval_queue::active() else {
        return false;
    };
    if matches!(key.code, KeyCode::Tab | KeyCode::BackTab) {
        approval_queue::cycle(
            key.code == KeyCode::Tab
                && !key
                    .modifiers
                    .contains(crossterm::event::KeyModifiers::SHIFT),
        );
        *state.scroll = 0;
        return true;
    }
    let scroll = state.scroll;
    *scroll = (*scroll).min(item.scroll_limit);
    match key.code {
        KeyCode::Up => *scroll = scroll.saturating_sub(1),
        KeyCode::Down => *scroll = scroll.saturating_add(1).min(item.scroll_limit),
        KeyCode::PageUp => *scroll = scroll.saturating_sub(10),
        KeyCode::PageDown => *scroll = scroll.saturating_add(10).min(item.scroll_limit),
        KeyCode::Home => *scroll = 0,
        KeyCode::End => *scroll = item.scroll_limit,
        _ => return false,
    }
    true
}
