//! Build display lines for the audit-loop view.

use ratatui::text::Line;

use super::audit_loop_state::AuditLoopState;

#[path = "audit_loop_line_parts.rs"]
mod parts;
use parts::{complete_line, gate_line, header, iteration_header};

/// Build the full set of lines: header summary, then each iteration.
pub(crate) fn build_lines(state: &AuditLoopState) -> Vec<Line<'static>> {
    let mut lines = vec![header(state), Line::from("")];
    for it in &state.iterations {
        lines.push(iteration_header(it));
        for gate in &it.gates {
            lines.push(gate_line(gate));
        }
        lines.push(Line::from(""));
    }
    if state.complete {
        lines.push(complete_line(state.passed));
    }
    lines
}
