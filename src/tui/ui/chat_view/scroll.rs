//! Scroll-offset clamping for the messages panel.
//!
//! Wrapping-aware: computes total rendered height by dividing each line's
//! width by the visible content width.

use ratatui::layout::Rect;
use ratatui::text::Line;

use crate::tui::app::state::App;

/// Sentinel meaning "follow latest content" (auto-scroll).
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::scroll::SCROLL_BOTTOM;
/// assert!(SCROLL_BOTTOM > 999_999);
/// ```
const SCROLL_BOTTOM: usize = 1_000_000;

/// Clamp scroll offset for `lines` inside `messages_rect`.
///
/// When [`AppState::chat_scroll`] >= [`SCROLL_BOTTOM`], snaps to bottom.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::scroll::clamp_scroll;
/// # fn d(a:&mut codetether_agent::tui::app::state::App){ let r=clamp_scroll(a,ratatui::layout::Rect::new(0,0,80,24),&vec![]); assert_eq!(r,0); }
/// ```
pub fn clamp_scroll(app: &mut App, messages_rect: Rect, lines: &[Line<'_>]) -> u16 {
    let content_width = messages_rect.width.saturating_sub(2) as usize;
    let total: usize = lines
        .iter()
        .map(|l| {
            let w = l.width();
            if w == 0 || content_width == 0 {
                1
            } else {
                w.div_ceil(content_width)
            }
        })
        .sum();
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
