//! Existing training conversion regression.
use super::*;

#[test]
fn test_tool_grouping_keeps_legacy_none_correlation_compatible() {
    let envelopes = vec![
        tool_request_envelope("env-1", "req-1", None),
        tool_request_envelope("env-2", "req-2", None),
        tool_response_envelope("env-3", "req-1", None),
        tool_response_envelope("env-4", "req-2", None),
    ];

    let records = collect::collect(&envelopes);
    let assistant_batches: Vec<_> = records
        .iter()
        .filter(|record| record.metadata.bus_kind == "tool_request_batch")
        .collect();

    assert_eq!(records.len(), 3);
    assert_eq!(assistant_batches.len(), 1);
    assert!(assistant_batches[0].metadata.correlation_id.is_none());
    assert_eq!(assistant_batches[0].tool_calls.as_ref().unwrap().len(), 2);
}
