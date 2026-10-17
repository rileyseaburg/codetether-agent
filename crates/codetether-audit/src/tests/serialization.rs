//! Backward-compatible event wire format and omitted optional fields.

use crate::AuditEntry;
use serde_json::json;

#[test]
fn legacy_entries_round_trip_without_optional_fields() {
    let wire = json!({
        "id": "event-1", "timestamp": "2026-01-01T00:00:00Z",
        "category": "tool_execution", "action": "run",
        "principal": null, "outcome": "denied"
    });
    let entry: AuditEntry = serde_json::from_value(wire.clone()).unwrap();
    assert!(entry.detail.is_none() && entry.duration_ms.is_none());
    assert!(entry.okr_id.is_none() && entry.okr_run_id.is_none());
    assert!(entry.relay_id.is_none() && entry.session_id.is_none());
    assert_eq!(serde_json::to_value(entry).unwrap(), wire);
}
