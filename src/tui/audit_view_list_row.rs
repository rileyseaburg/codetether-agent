use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::audit::{AuditEntry, AuditOutcome};

#[path = "audit_view_list_text.rs"]
mod text;

pub(super) fn format(entry: &AuditEntry) -> Line<'static> {
    let (mark, color) = outcome(entry.outcome);
    Line::from(vec![
        Span::styled(
            entry.timestamp.format("%H:%M:%S").to_string(),
            Style::default().fg(Color::DarkGray),
        ),
        " ".into(),
        Span::styled(mark, Style::default().fg(color)),
        " ".into(),
        Span::styled(
            format!("{:<10}", text::category(entry.category)),
            Style::default().fg(Color::Cyan),
        ),
        " ".into(),
        Span::raw(text::truncate(&entry.action, 40)),
        " ".into(),
        Span::styled(
            format!("[{}]", entry.principal.as_deref().unwrap_or("-")),
            Style::default().fg(Color::DarkGray),
        ),
    ])
}

fn outcome(value: AuditOutcome) -> (&'static str, Color) {
    match value {
        AuditOutcome::Success => ("✓", Color::Green),
        AuditOutcome::Failure => ("✗", Color::Red),
        AuditOutcome::Denied => ("⊘", Color::Yellow),
    }
}
