//! Presentation data for the protocol summary panel.

use ratatui::style::Color;

use super::{
    render_summary_status as status, render_summary_text as text, state::BusLogState,
    summary::ProtocolSummary,
};

pub(super) struct SummaryData {
    pub(super) worker_label: &'static str,
    pub(super) worker_color: Color,
    pub(super) processing_label: &'static str,
    pub(super) processing_color: Color,
    pub(super) peer_label: &'static str,
    pub(super) peer_color: Color,
    pub(super) worker_id: String,
    pub(super) worker_name: String,
    pub(super) a2a_count: usize,
    pub(super) queued_tasks: usize,
    pub(super) registered_agents: String,
    pub(super) cwd_display: String,
    pub(super) recent_task: String,
}

pub(super) fn data(summary: &ProtocolSummary, state: &BusLogState) -> SummaryData {
    let (worker_label, worker_color) = status::worker(summary.a2a_connected);
    let (peer_label, peer_color) = status::peer(summary.peer_endpoint_ready);
    let (processing_label, processing_color) = status::processing(summary.processing);
    SummaryData {
        worker_label,
        worker_color,
        processing_label,
        processing_color,
        peer_label,
        peer_color,
        worker_id: text::worker_id(summary),
        worker_name: text::worker_name(summary),
        a2a_count: state.a2a_message_count(),
        queued_tasks: summary.queued_tasks,
        registered_agents: text::registered_agents(summary),
        cwd_display: summary.cwd_display.clone(),
        recent_task: text::recent_task(summary),
    }
}
