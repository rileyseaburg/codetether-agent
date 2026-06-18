//! Render the audit-loop TUI view (`/auditloop`).
//!
//! Visualizes the implement → audit → retry cycle so the user can watch each
//! iteration's gates pass or fail rather than trusting the agent blindly.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

#[path = "audit_loop_event.rs"]
pub mod audit_loop_event;
#[path = "audit_loop_lines.rs"]
mod audit_loop_lines;
#[path = "audit_loop_model.rs"]
pub mod audit_loop_model;
#[path = "audit_loop_reduce.rs"]
mod audit_loop_reduce;
#[path = "audit_loop_state.rs"]
pub mod audit_loop_state;

#[cfg(test)]
#[path = "audit_loop_tests.rs"]
mod tests;

use crate::tui::input::render_input_preview;
use audit_loop_state::AuditLoopState;

/// Render the audit-loop view into `area`.
pub fn render_audit_loop_view(f: &mut Frame, state: &AuditLoopState, area: Rect, status: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(6), Constraint::Length(3)])
        .split(area);

    let widget = Paragraph::new(audit_loop_lines::build_lines(state))
        .block(Block::default().borders(Borders::ALL).title("Audit Loop"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_input_preview(f, chunks[1], status);
}
