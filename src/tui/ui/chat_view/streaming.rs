//! In-flight streaming assistant preview.
//!
//! Renders partial text via full [`MessageFormatter`]. Uses a thread-local
//! parse cache to avoid re-running the markdown formatter on every token:
//! while the streaming text grows by fewer than [`STREAM_REPARSE_THRESHOLD`]
//! bytes since the last parse, the cached lines are reused as-is.

use std::cell::RefCell;

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::tui::app::state::AppState;
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ui::status_bar::format_timestamp;

/// Reparse threshold: reuse the previously-parsed streaming preview while the
/// text has grown by fewer than this many bytes.
const STREAM_REPARSE_THRESHOLD: usize = 48;

thread_local! {
    /// `(streaming_text_len_at_parse, parsed_lines)`. Keyed per UI thread;
    /// the TUI renders single-threaded so this avoids any locking cost.
    static STREAM_PARSE_CACHE: RefCell<Option<(usize, Vec<Line<'static>>)>> =
        const { RefCell::new(None) };
}

/// Append a streaming preview block when the app is actively receiving text.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::streaming::push_streaming_preview;
/// # fn d(s:&codetether_agent::tui::app::state::AppState){ let f=codetether_agent::tui::message_formatter::MessageFormatter::new(76); let mut l:Vec<ratatui::text::Line>=vec![]; push_streaming_preview(&mut l,s,40,&f); }
/// ```
pub fn push_streaming_preview(
    lines: &mut Vec<Line<'static>>,
    state: &AppState,
    separator_width: usize,
    formatter: &MessageFormatter,
) {
    if !state.processing || state.streaming_text.is_empty() {
        return;
    }
    lines.push(Line::from(Span::styled(
        "─".repeat(separator_width.min(40)),
        Style::default().fg(Color::DarkGray).dim(),
    )));
    lines.push(Line::from(vec![
        Span::styled(
            format!("[{}] ", format_timestamp(std::time::SystemTime::now())),
            Style::default().fg(Color::DarkGray).dim(),
        ),
        Span::styled("◆ ", Style::default().fg(Color::Cyan).bold()),
        Span::styled("assistant", Style::default().fg(Color::Cyan).bold()),
        Span::styled(
            " (streaming…)",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
        ),
        super::elapsed_badge::elapsed_badge(state),
    ]));
    let formatted = cached_format(&state.streaming_text, formatter);
    for line in formatted {
        let mut spans = vec![Span::styled("  ", Style::default().fg(Color::Cyan))];
        spans.extend(line.spans);
        lines.push(Line::from(spans));
    }
}

fn cached_format(text: &str, formatter: &MessageFormatter) -> Vec<Line<'static>> {
    STREAM_PARSE_CACHE.with(|cell| {
        let cur_len = text.len();
        if let Some((parsed_len, ref lines)) = *cell.borrow()
            && cur_len >= parsed_len
            && cur_len - parsed_len < STREAM_REPARSE_THRESHOLD
        {
            return lines.clone();
        }
        let formatted = formatter.format_content(text, "assistant");
        *cell.borrow_mut() = Some((cur_len, formatted.clone()));
        formatted
    })
}

/// Reset the streaming parse cache. Call when a new assistant turn begins
/// (e.g. `streaming_text` was cleared) so a shrunken buffer doesn't keep
/// reusing stale parsed lines.
pub fn reset_stream_parse_cache() {
    STREAM_PARSE_CACHE.with(|cell| *cell.borrow_mut() = None);
}
