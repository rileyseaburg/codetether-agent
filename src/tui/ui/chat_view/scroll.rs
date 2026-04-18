//! Scroll-offset clamping for the messages panel.
//!
//! The chat [`Paragraph`] is rendered without `Wrap`, so ratatui draws
//! exactly one terminal row per [`Line`]. Total height is therefore just
//! `lines.len()` — no per-line width arithmetic required.
//!
//! [`Paragraph`]: ratatui::widgets::Paragraph

use ratatui::layout::Rect;
use ratatui::text::Line;

use crate::tui::app::state::App;

/// Sentinel meaning "follow latest content" (auto-scroll).
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::scroll::SCROLL_BOTTOM;
/// assert!(SCROLL_BOTTOM > 999_999);
/// ```
pub const SCROLL_BOTTOM: usize = 1_000_000;

/// No-op retained for API compatibility with an earlier memo-based
/// revision of the scroll-height calculator.
///
/// The current implementation recomputes the total in O(1) from
/// `lines.len()`, so there is no memo to reset. Callers that still invoke
/// this function continue to compile and behave identically.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::scroll::reset_height_memo;
/// reset_height_memo(); // always safe, never fails
/// ```
pub fn reset_height_memo() {}

/// Clamp scroll offset so the visible slice of `lines` stays within
/// `messages_rect` and honor the `SCROLL_BOTTOM` follow-tail sentinel.
///
/// The chat `Paragraph` is rendered without wrapping (see module docs),
/// so the total rendered height equals `lines.len()` and no per-line
/// width arithmetic is needed.
///
/// # Arguments
///
/// * `app` — mutable app state; `chat_scroll` is read and the computed
///   `max_scroll` is written via `AppState::set_chat_max_scroll`.
/// * `messages_rect` — the panel rect including borders; interior height
///   is `height - 2`.
/// * `lines` — the rendered chat lines for this frame.
///
/// # Returns
///
/// The clamped scroll offset suitable for `Paragraph::scroll((offset, 0))`.
/// Saturates at `u16::MAX` for safety on very tall buffers.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::scroll::clamp_scroll;
/// # fn d(a:&mut codetether_agent::tui::app::state::App){
/// let r = clamp_scroll(a, ratatui::layout::Rect::new(0, 0, 80, 24), &vec![]);
/// assert_eq!(r, 0);
/// # }
/// ```
pub fn clamp_scroll(app: &mut App, messages_rect: Rect, lines: &[Line<'_>]) -> u16 {
    // Paragraph is rendered without Wrap → 1 line == 1 terminal row.
    let total = lines.len();
    let visible = messages_rect.height.saturating_sub(2) as usize;
    let max_scroll = total.saturating_sub(visible);
    app.state.set_chat_max_scroll(max_scroll);
    let scroll = if app.state.chat_scroll >= SCROLL_BOTTOM {
        max_scroll
    } else {
        app.state.chat_scroll.min(max_scroll)
    };
    scroll.min(u16::MAX as usize) as u16
}
