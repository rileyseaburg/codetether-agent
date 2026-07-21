//! Cached chat-line buffer builder.
//!
//! [`build_chat_lines`] returns a width-keyed cached vector of [`Line`]s,
//! delegating to [`build_uncached`] for cold-cache rebuilds. Uses `take`
//! semantics on the cache hit path to avoid cloning the entire line buffer.

use crate::tui::app::state::App;
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;

use super::build_uncached::build_uncached;
pub use super::drawn_lines::DrawnLines;
use super::streaming::push_streaming_preview;

/// Build (or take cached) chat lines for the current width.
///
/// On the cache-hit path, takes ownership of the cached lines (zero-clone).
/// The caller must call [`DrawnLines::restore`] after rendering to put the
/// lines back into the cache.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::lines::build_chat_lines;
/// # fn d(a:&mut codetether_agent::tui::app::state::App){ let p=codetether_agent::tui::color_palette::ColorPalette::marketing(); let f=codetether_agent::tui::message_formatter::MessageFormatter::new(76); let l=build_chat_lines(a,80,80,&f,&p); }
/// ```
pub fn build_chat_lines(
    app: &mut App,
    max_width: usize,
    content_width: usize,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) -> DrawnLines {
    if let Some(lines) = super::focused_lines::build(app, content_width, formatter, palette) {
        return lines;
    }
    // Hot path: cache is valid — take ownership, no clone.
    if let Some(lines) = app.state.take_cached_if_valid(max_width) {
        return DrawnLines::from_cache(lines);
    }

    let separator_width = content_width.saturating_sub(2).min(60);
    let panel_width = content_width.saturating_sub(4);

    // Warm path: frozen prefix from prior frame + streaming suffix.
    if let Some(mut prefix) = app.state.take_frozen_prefix(max_width) {
        let frozen_len = prefix.len();
        push_streaming_preview(&mut prefix, &app.state, separator_width, formatter);
        return store_and_take(app, prefix, max_width, frozen_len);
    }

    // Cold path: full rebuild.
    let mut built = build_uncached(app, separator_width, panel_width, formatter, palette);
    let frozen_len = built.len();
    push_streaming_preview(&mut built, &app.state, separator_width, formatter);
    store_and_take(app, built, max_width, frozen_len)
}

/// Store freshly built lines in the cache, then take them back out for this
/// frame via the zero-clone take/restore path.
///
/// This avoids cloning the entire line buffer every streaming frame: the
/// vector is moved into the cache, then moved straight back out for rendering
/// and restored after the draw. For long conversations this removes thousands
/// of per-frame [`Line`] clones that previously caused visible input lag.
fn store_and_take(
    app: &mut App,
    lines: Vec<ratatui::text::Line<'static>>,
    max_width: usize,
    frozen_len: usize,
) -> DrawnLines {
    app.state
        .store_message_lines_with_frozen(lines, max_width, frozen_len);
    match app.state.take_cached_if_valid(max_width) {
        Some(taken) => DrawnLines::from_cache(taken),
        // Cache predicate rejected reuse (e.g. pending tool timer); fall back
        // to a clone so this frame still renders.
        None => DrawnLines::from_rebuild(app.state.cached_message_lines.clone()),
    }
}
