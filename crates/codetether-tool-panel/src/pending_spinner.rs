//! In-flight tool spinner row appended to the activity panel.

use std::time::Instant;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::arg_preview::smart_arg_preview;

pub(super) fn append_pending_tool(
    body_lines: &mut Vec<Line<'static>>,
    name: &str,
    started_at: Instant,
    arguments: Option<&str>,
    approval_tool: impl FnOnce() -> Option<String>,
) {
    let elapsed = started_at.elapsed().as_secs_f64();
    let label = pending_label(name, elapsed, approval_tool);
    body_lines.push(Line::from(vec![
        Span::styled("│ ", Style::default().fg(Color::DarkGray).dim()),
        Span::styled(label, Style::default().fg(Color::Yellow).italic()),
    ]));
    if let Some(args) = arguments {
        let preview = smart_arg_preview(name, args);
        if !preview.is_empty() {
            body_lines.push(Line::from(vec![
                Span::styled("│   ", Style::default().fg(Color::DarkGray).dim()),
                Span::styled(preview, Style::default().fg(Color::Yellow).dim()),
            ]));
        }
    }
}

fn pending_label(
    name: &str,
    elapsed: f64,
    approval_tool: impl FnOnce() -> Option<String>,
) -> String {
    match approval_tool() {
        Some(tool) if tool == name => format!("⏳ {name} … awaiting approval ({elapsed:.1}s)"),
        _ => format!("⏳ {name} … running ({elapsed:.1}s)"),
    }
}
