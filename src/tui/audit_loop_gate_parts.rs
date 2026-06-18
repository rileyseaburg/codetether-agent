//! Gate and completion line builders for the audit-loop view.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use super::super::super::audit_loop_model::{Gate, GateStatus};

/// One gate row: glyph, name, optional detail.
pub(crate) fn gate_line(gate: &Gate) -> Line<'static> {
    let (glyph, color) = match gate.status {
        GateStatus::Pending => ("○", Color::DarkGray),
        GateStatus::Running => ("◐", Color::Yellow),
        GateStatus::Passed => ("✔", Color::Green),
        GateStatus::Failed => ("✘", Color::Red),
    };
    let mut spans = vec![
        Span::styled(format!("   {glyph} "), Style::default().fg(color)),
        Span::raw(gate.name.clone()),
    ];
    if let Some(d) = &gate.detail {
        spans.push(Span::styled(
            format!("  — {d}"),
            Style::default().fg(Color::DarkGray),
        ));
    }
    Line::from(spans)
}

/// Final pass/fail banner.
pub(crate) fn complete_line(passed: bool) -> Line<'static> {
    let (text, color) = if passed {
        ("✔ loop complete — all gates passed", Color::Green)
    } else {
        ("✘ loop ended — gates still failing", Color::Red)
    };
    Line::from(Span::styled(
        text,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))
}
