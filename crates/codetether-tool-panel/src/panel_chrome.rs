//! Header / footer / placeholder rows surrounding the panel body.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use codetether_util::preview_chars::truncate_preview;

pub(super) fn panel_header(
    timestamp: &str,
    count: usize,
    start: usize,
    end: usize,
    total: usize,
    max_scroll: usize,
    width: usize,
) -> Line<'static> {
    let scroll_label = if max_scroll == 0 {
        format!("{total} lines")
    } else {
        format!("{}-{end} / {total}", start + 1)
    };
    let header = format!("[{timestamp}] ▣ tools {count} • {scroll_label}");
    Line::from(Span::styled(
        truncate_preview(&header, width),
        Style::default().fg(Color::Cyan).bold(),
    ))
}

pub(super) fn panel_footer(start: usize, max_scroll: usize, width: usize) -> Line<'static> {
    let footer = if max_scroll == 0 {
        "└ ready".to_string()
    } else {
        format!("└ preview scroll {}", start + 1)
    };
    Line::from(Span::styled(
        truncate_preview(&footer, width),
        Style::default().fg(Color::DarkGray).dim(),
    ))
}

pub(super) use super::panel_empty::empty_body_lines;
