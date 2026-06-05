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
    // Hot path: cache is valid — take ownership, no clone.
    if let Some(lines) = app.state.take_cached_if_valid(max_width) {
        return DrawnLines::from_cache(lines);
    }

    let separator_width = content_width.saturating_sub(2).min(60);
    let panel_width = content_width.saturating_sub(4);

    // Warm path: frozen prefix from prior frame + streaming suffix.
    if let Some(mut prefix) = app.state.clone_frozen_prefix(max_width) {
        let frozen_len = prefix.len();
        push_streaming_preview(&mut prefix, &app.state, separator_width, formatter);
        app.state
            .store_message_lines_with_frozen(prefix.clone(), max_width, frozen_len);
        return DrawnLines::from_rebuild(prefix);
    }

    // Cold path: full rebuild.
    let mut built = build_uncached(app, separator_width, panel_width, formatter, palette);
    let frozen_len = built.len();
    push_streaming_preview(&mut built, &app.state, separator_width, formatter);
    app.state
        .store_message_lines_with_frozen(built.clone(), max_width, frozen_len);
    DrawnLines::from_rebuild(built)
}
