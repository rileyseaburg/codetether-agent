//! Snapshot native status and approval state for shared badge formatting.
use crate::tui::app::state::{App, approval_queue};
use codetether_webview_bars::prefix::{Approval, Status};
use ratatui::text::Span;
pub fn build(app: &App) -> Vec<Span<'static>> {
    let request = approval_queue::active();
    let approval = request.as_ref().map(|request| Approval {
        id: &request.id,
        pending: approval_queue::len(),
    });
    codetether_webview_bars::prefix::build(Status {
        processing: app.state.processing,
        status: &app.state.status,
        pending_tool: app.state.tool_timing.pending_tool_name.as_deref(),
        approval,
    })
}
