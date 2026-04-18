//! Cached chat-line buffer builder.
//!
//! [`build_chat_lines`] returns a width-keyed cached vector of [`Line`]s,
//! delegating to [`append_entries`] and [`push_streaming_preview`].

use ratatui::text::Line;

use crate::tui::app::state::App;
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ui::tool_panel::build_render_entries;

use super::empty::push_empty_placeholder;
use super::entries::append_entries;
use super::streaming::push_streaming_preview;

/// Build (or return cached) chat lines for the current width.
///
/// Uses [`AppState::get_or_build_message_lines`] as cache key.
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
) -> Vec<Line<'static>> {
    if let Some(cached) = app.state.get_or_build_message_lines(max_width) {
        return cached;
    }
    let separator_width = content_width.saturating_sub(2).min(60);
    let panel_width = content_width.saturating_sub(4);

    // Fast path: messages / width / processing unchanged — only streaming_text
    // differs. Reuse the frozen prefix and rebuild just the streaming suffix.
    if let Some(mut lines) = app.state.clone_frozen_prefix(max_width) {
        let frozen_len = lines.len();
        push_streaming_preview(&mut lines, &app.state, separator_width, formatter);
        app.state
            .store_message_lines_with_frozen(lines.clone(), max_width, frozen_len);
        return lines;
    }

    let mut lines = Vec::new();
    let entries = build_render_entries(&app.state.messages);
    if entries.is_empty() {
        push_empty_placeholder(&mut lines);
        app.state.set_tool_preview_max_scroll(0);
    } else {
        let result = append_entries(
            &mut lines,
            &entries,
            separator_width,
            panel_width,
            app.state.tool_preview_scroll,
            formatter,
            palette,
        );
        app.state
            .set_tool_preview_max_scroll(result.tool_preview_max_scroll);
    }
    let frozen_len = lines.len();
    push_streaming_preview(&mut lines, &app.state, separator_width, formatter);
    app.state
        .store_message_lines_with_frozen(lines.clone(), max_width, frozen_len);
    lines
}
