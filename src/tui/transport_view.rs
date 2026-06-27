//! Transport-health view: renders the latest `TCP_INFO` probe snapshot.
//!
//! Surfaces RTT, RTT variance, congestion window, and retransmit counters from
//! `TRANSPORT_METRICS` so path degradation is visible as a first-class reading,
//! distinct from process-local latency. See
//! `docs/transport-first-class-plan.md` Phase 4.

use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::telemetry::TRANSPORT_METRICS;

/// Render the transport-health panel into `area`.
pub fn render_transport(f: &mut Frame, area: Rect) {
    let mut lines = vec![
        Line::from("Transport Health (TCP_INFO probe)"),
        Line::from(""),
    ];
    match TRANSPORT_METRICS.latest() {
        Some(s) => {
            lines.push(Line::from(format!(
                "RTT {:.2} ms (var {:.2} ms)",
                s.rtt_us as f64 / 1000.0,
                s.rttvar_us as f64 / 1000.0
            )));
            lines.push(Line::from(format!(
                "Congestion window: {} segments",
                s.snd_cwnd
            )));
            lines.push(Line::from(format!(
                "Retransmits: {} in-flight / {} total",
                s.retrans, s.total_retrans
            )));
            if s.total_retrans > 0 {
                lines.push(Line::from(
                    "Note: retransmits indicate packet loss on the path.",
                ));
            }
        }
        None => lines.push(Line::from(
            "No probe sample yet (Linux-only; 15s interval).",
        )),
    }
    let block = Block::default().borders(Borders::ALL).title("Transport");
    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}
