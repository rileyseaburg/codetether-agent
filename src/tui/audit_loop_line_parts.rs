//! Header line builders for the audit-loop view.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use super::super::audit_loop_model::{Iteration, IterationStatus};
use super::super::audit_loop_state::AuditLoopState;

#[path = "audit_loop_gate_parts.rs"]
mod gate_parts;
pub(crate) use gate_parts::{complete_line, gate_line};

/// Top summary line: task and iteration count.
pub(super) fn header(state: &AuditLoopState) -> Line<'static> {
    let n = state.iterations.len();
    Line::from(vec![
        Span::styled(
            "Audit loop: ",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(state.task.clone()),
        Span::styled(
            format!("  ({n}/{} iterations)", state.max_iterations),
            Style::default().fg(Color::DarkGray),
        ),
    ])
}

/// Heading for one iteration with a status glyph.
pub(super) fn iteration_header(it: &Iteration) -> Line<'static> {
    let (glyph, color) = match it.status {
        IterationStatus::Running => ("▶", Color::Yellow),
        IterationStatus::Passed => ("✔", Color::Green),
        IterationStatus::Failed => ("✘", Color::Red),
    };
    Line::from(vec![
        Span::styled(
            format!("{glyph} #{} ", it.number),
            Style::default().fg(color),
        ),
        Span::styled(
            it.summary.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])
}
