//! Scroll-offset clamping for the messages panel.
//!
//! Wrapping-aware: computes total rendered height by dividing each line's
//! width by the visible content width. Uses a thread-local memo so that
//! frames where only the trailing (streaming) line changed skip the full
//! O(N) sum.

use std::cell::RefCell;

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

thread_local! {
    /// (lines_len, content_width, prefix_height) for the last successful compute.
    /// `prefix_height` is the wrapped-height sum of lines[..lines_len-1]
    /// (everything except the final line). When the next call matches the
    /// first two, we add only the last line's wrapped height.
    static HEIGHT_MEMO: RefCell<Option<(usize, usize, usize)>> =
        const { RefCell::new(None) };
}

fn wrapped_height(line_width: usize, content_width: usize) -> usize {
    if line_width == 0 || content_width == 0 {
        1
    } else {
        line_width.div_ceil(content_width)
    }
}

fn total_height(lines: &[Line<'_>], content_width: usize) -> usize {
    let len = lines.len();
    if len == 0 {
        return 0;
    }
    let last_h = wrapped_height(lines[len - 1].width(), content_width);

    let memo = HEIGHT_MEMO.with(|c| *c.borrow());
    if let Some((cached_len, cached_width, prefix)) = memo
        && cached_len == len
        && cached_width == content_width
    {
        return prefix + last_h;
    }

    // Full recompute: sum all-but-last, store prefix, return total.
    let prefix: usize = lines[..len - 1]
        .iter()
        .map(|l| wrapped_height(l.width(), content_width))
        .sum();
    HEIGHT_MEMO.with(|c| *c.borrow_mut() = Some((len, content_width, prefix)));
    prefix + last_h
}

/// Reset the height memo. Call when width changes or on full rebuild.
pub fn reset_height_memo() {
    HEIGHT_MEMO.with(|c| *c.borrow_mut() = None);
}

/// Clamp scroll offset for `lines` inside `messages_rect`.
///
/// When `AppState::chat_scroll >= SCROLL_BOTTOM`, snaps to bottom.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::scroll::clamp_scroll;
/// # fn d(a:&mut codetether_agent::tui::app::state::App){ let r=clamp_scroll(a,ratatui::layout::Rect::new(0,0,80,24),&vec![]); assert_eq!(r,0); }
/// ```
pub fn clamp_scroll(app: &mut App, messages_rect: Rect, lines: &[Line<'_>]) -> u16 {
    let content_width = messages_rect.width.saturating_sub(2) as usize;
    let total = total_height(lines, content_width);
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
