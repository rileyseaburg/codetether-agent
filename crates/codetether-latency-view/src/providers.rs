//! Provider latency ranking and six-row presentation.
use crate::format::{format_duration, format_tps, section_heading};
use codetether_telemetry::PROVIDER_METRICS;
use ratatui::text::Line;
pub(crate) fn append(lines: &mut Vec<Line<'static>>) {
    lines.push(Line::from(""));
    lines.push(section_heading("Provider Round-Trip Latency"));

    let mut provider_snapshots = PROVIDER_METRICS.all_snapshots();
    provider_snapshots.sort_by(|a, b| {
        b.request_count
            .cmp(&a.request_count)
            .then_with(|| b.avg_latency_ms.total_cmp(&a.avg_latency_ms))
            .then_with(|| a.provider.cmp(&b.provider))
    });
    if provider_snapshots.is_empty() {
        lines.push(Line::from("  No provider requests recorded yet."));
    } else {
        for snapshot in provider_snapshots.iter().take(6) {
            lines.push(Line::from(format!(
                "  {}: avg {} | p50 {} | p95 {} | {} reqs | {} avg TPS",
                snapshot.provider,
                format_duration(snapshot.avg_latency_ms.round() as u64),
                format_duration(snapshot.p50_latency_ms.round() as u64),
                format_duration(snapshot.p95_latency_ms.round() as u64),
                snapshot.request_count,
                format_tps(snapshot.avg_tps)
            )));
        }
    }
}
