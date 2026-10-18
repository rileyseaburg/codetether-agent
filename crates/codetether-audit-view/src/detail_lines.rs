//! Selected audit entry metadata and JSON lines.
use crate::key_value::kv_line;
use codetether_audit::{AuditEntry, AuditOutcome};
use ratatui::{style::Stylize, text::Line};
pub(crate) fn lines(entry: &AuditEntry) -> Vec<Line<'static>> {
    let detail_json = entry
        .detail
        .as_ref()
        .map(|v| serde_json::to_string_pretty(v).unwrap_or_else(|_| v.to_string()))
        .unwrap_or_else(|| "(none)".to_string());

    vec![
        kv_line("id", &entry.id),
        kv_line("time", &entry.timestamp.format("%H:%M:%S%.3f").to_string()),
        kv_line("category", &format!("{:?}", entry.category)),
        kv_line("action", &entry.action),
        kv_line(
            "outcome",
            match entry.outcome {
                AuditOutcome::Success => "success",
                AuditOutcome::Failure => "failure",
                AuditOutcome::Denied => "denied",
            },
        ),
        kv_line("principal", entry.principal.as_deref().unwrap_or("-")),
        kv_line("session", entry.session_id.as_deref().unwrap_or("-")),
        kv_line(
            "duration_ms",
            &entry
                .duration_ms
                .map(|d| d.to_string())
                .unwrap_or_else(|| "-".into()),
        ),
        Line::from(""),
        "detail:".dim().into(),
        Line::from(detail_json),
    ]
}
