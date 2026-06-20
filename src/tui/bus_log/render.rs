//! Top-level bus log rendering entrypoints.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use super::{
    render_body::render_bus_body, render_summary, state::BusLogState, summary::ProtocolSummary,
};

/// Renders the protocol bus log without a summary panel.
///
/// # Arguments
///
/// * `f` - Ratatui frame to draw into.
/// * `state` - Mutable bus log state, including list selection.
/// * `area` - Terminal area reserved for the bus log.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::bus_log::{BusLogState, render_bus_log};
/// use ratatui::{Frame, layout::Rect};
///
/// fn draw(f: &mut Frame, area: Rect) {
///     let mut state = BusLogState::new();
///     render_bus_log(f, &mut state, area);
/// }
/// ```
pub fn render_bus_log(f: &mut Frame, state: &mut BusLogState, area: Rect) {
    render_bus_log_with_summary(f, state, area, None);
}

/// Renders the protocol bus log and optional protocol summary panel.
///
/// # Arguments
///
/// * `f` - Ratatui frame to draw into.
/// * `state` - Mutable bus log state, including list selection.
/// * `area` - Terminal area reserved for the protocol view.
/// * `summary` - Optional protocol status rendered above the log.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::bus_log::{
///     BusLogState, ProtocolSummary, render_bus_log_with_summary,
/// };
/// use ratatui::{Frame, layout::Rect};
///
/// fn draw(f: &mut Frame, area: Rect) {
///     let mut state = BusLogState::new();
///     let summary = ProtocolSummary {
///         cwd_display: ".".to_string(),
///         worker_id: None,
///         worker_name: None,
///         a2a_connected: false,
///         processing: None,
///         registered_agents: Vec::new(),
///         queued_tasks: 0,
///         recent_task: None,
///         peer_endpoint_ready: false,
///     };
///     render_bus_log_with_summary(f, &mut state, area, Some(summary));
/// }
/// ```
pub fn render_bus_log_with_summary(
    f: &mut Frame,
    state: &mut BusLogState,
    area: Rect,
    summary: Option<ProtocolSummary>,
) {
    if let Some(summary) = summary {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8),
                Constraint::Min(8),
                Constraint::Length(2),
            ])
            .split(area);
        render_summary::render_summary_panel(f, state, chunks[0], &summary);
        render_bus_body(f, state, chunks[1], chunks[2]);
    } else {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(8), Constraint::Length(2)])
            .split(area);
        render_bus_body(f, state, chunks[0], chunks[1]);
    }
}
