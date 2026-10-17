//! Existing training conversion regression.
use super::*;

#[test]
fn test_tool_grouping_keeps_different_correlation_ids_separate() {
    let envelopes = vec![
        tool_request_envelope("env-1", "req-1", Some("turn-1")),
        tool_response_envelope("env-2", "req-1", Some("turn-1")),
        tool_request_envelope("env-3", "req-2", Some("turn-2")),
        tool_response_envelope("env-4", "req-2", Some("turn-2")),
    ];

    let records = collect::collect(&envelopes);
    let assistant_batches: Vec<_> = records
        .iter()
        .filter(|record| record.metadata.bus_kind == "tool_request_batch")
        .collect();

    assert_eq!(records.len(), 4);
    assert_eq!(assistant_batches.len(), 2);
    assert_eq!(
        assistant_batches[0].metadata.correlation_id.as_deref(),
        Some("turn-1")
    );
    assert_eq!(
        assistant_batches[1].metadata.correlation_id.as_deref(),
        Some("turn-2")
    );
    assert_eq!(assistant_batches[0].tool_calls.as_ref().unwrap().len(), 1);
    assert_eq!(assistant_batches[1].tool_calls.as_ref().unwrap().len(), 1);
}
