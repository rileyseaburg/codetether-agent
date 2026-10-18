//! History anchoring and final message-panel scroll clamping.
use super::Scroll;
use crate::SCROLL_BOTTOM;
use ratatui::{layout::Rect, text::Line};
/// Compatibility no-op for callers of the former height memo.
/// # Examples
/// ```
/// codetether_chat_layout::messages::reset_height_memo();
/// ```
pub fn reset_height_memo() {}
/// Clamp to visible bounds after applying pending history anchoring.
/// # Arguments
/// Receives history/viewport access, panel geometry and unwrapped rendered rows.
/// # Returns
/// A terminal scroll offset saturated at u16::MAX.
/// # Examples
/// ```no_run
/// fn clamp<S: codetether_chat_layout::messages::Scroll>(state: &mut S) -> u16 {
///     codetether_chat_layout::messages::clamp(state, ratatui::layout::Rect::new(0, 0, 80, 24), &[])
/// }
/// ```
pub fn clamp(host: &mut impl Scroll, messages_rect: Rect, lines: &[Line<'_>]) -> u16 {
    let total = lines.len();
    let visible = messages_rect.height.saturating_sub(2) as usize;
    host.set_viewport_height(visible);
    host.apply_pending_anchor(total);
    let max_scroll = total.saturating_sub(visible);
    let viewport = host.viewport();
    viewport.set_chat_max_scroll(max_scroll);
    let scroll = if viewport.chat_scroll >= SCROLL_BOTTOM {
        max_scroll
    } else {
        viewport.chat_scroll.min(max_scroll)
    };
    scroll.min(u16::MAX as usize) as u16
}
