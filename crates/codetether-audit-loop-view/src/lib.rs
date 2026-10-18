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

use audit_loop_state::AuditLoopState;

/// Render the audit-loop view into `area`.
/// # Arguments
/// * `f` - Target frame.
/// * `state` - Current reduced audit-loop state.
/// * `area` - Available terminal rectangle.
/// * `status` - Footer status text.
/// * `render_input_preview` - Host footer rendering callback.
/// # Returns
/// Unit after rendering the state and footer.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let state = codetether_audit_loop_view::audit_loop_state::AuditLoopState::new();
/// codetether_audit_loop_view::render_audit_loop_view(f, &state, area, "ready", |_, _, _| {});
/// # }
/// ```
pub fn render_audit_loop_view(
    f: &mut Frame,
    state: &AuditLoopState,
    area: Rect,
    status: &str,
    render_input_preview: impl FnOnce(&mut Frame, Rect, &str),
) {
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

mod state_apply;
mod state_start;
