//! Tool panel scroll-window math and blank row padding.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::TOOL_PANEL_VISIBLE_LINES;
use crate::tui::app::state::scroll::TOOL_PREVIEW_FOLLOW;

pub(super) struct PanelWindow {
    pub start: usize,
    pub end: usize,
    pub total: usize,
    pub max_scroll: usize,
    pub visible_lines: usize,
}

pub(super) fn panel_window(total: usize, scroll_offset: usize) -> PanelWindow {
    let visible_lines = TOOL_PANEL_VISIBLE_LINES.max(1);
    let max_scroll = total.saturating_sub(visible_lines);
    let start = if scroll_offset >= TOOL_PREVIEW_FOLLOW {
        max_scroll
    } else {
        scroll_offset.min(max_scroll)
    };
    let end = (start + visible_lines).min(total);
    PanelWindow {
        start,
        end,
        total,
        max_scroll,
        visible_lines,
    }
}

pub(super) fn pad_blank_rows(lines: &mut Vec<Line<'static>>, count: usize) {
    for _ in 0..count {
        lines.push(Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
            Span::raw(""),
        ]));
    }
}
