//! Bind the shared audit-loop view to the native input preview.
pub use crate::{audit_loop_event, audit_loop_model, audit_loop_state};
use audit_loop_state::AuditLoopState;
use codetether_tui_style::input_preview::render_input_preview;
use ratatui::{Frame, layout::Rect};
/// Render audit-loop progress using the shared state and line builders.
/// # Arguments
/// Receives the target frame, panel bounds and the current view data.
/// # Returns
/// Draws the panel using the shared input-preview presentation.
/// # Examples
/// ```no_run
/// # fn draw(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// codetether_audit_loop_view::standard::render_audit_loop_view(frame, &Default::default(), area, "Ready");
/// # }
/// ```
pub fn render_audit_loop_view(f: &mut Frame, state: &AuditLoopState, area: Rect, status: &str) {
    crate::render_audit_loop_view(f, state, area, status, render_input_preview);
}
