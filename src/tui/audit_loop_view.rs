//! Bind the shared audit-loop view to the native input preview.
use crate::tui::input::render_input_preview;
use audit_loop_state::AuditLoopState;
pub use codetether_audit_loop_view::{audit_loop_event, audit_loop_model, audit_loop_state};
use ratatui::{Frame, layout::Rect};
/// Render audit-loop progress using the shared state and line builders.
pub fn render_audit_loop_view(f: &mut Frame, state: &AuditLoopState, area: Rect, status: &str) {
    codetether_audit_loop_view::render_audit_loop_view(
        f,
        state,
        area,
        status,
        render_input_preview,
    );
}
