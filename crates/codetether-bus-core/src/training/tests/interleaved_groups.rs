//! Complete tool groups may interleave requests with earlier calls' responses.
use super::*;

fn ordered(mut envelopes: Vec<BusEnvelope>) -> Vec<BusEnvelope> {
    let start = Utc::now();
    for (index, envelope) in envelopes.iter_mut().enumerate() {
        envelope.timestamp = start + chrono::Duration::milliseconds(index as i64);
    }
    envelopes
}

#[test]
fn preserves_four_complete_interleaved_calls() {
    let mut envelopes = Vec::new();
    for index in 0..4 {
        let id = format!("req-{index}");
        envelopes.push(tool_request_envelope(
            &format!("request-{index}"),
            &id,
            Some("turn"),
        ));
        envelopes.push(tool_response_envelope(
            &format!("response-{index}"),
            &id,
            Some("turn"),
        ));
    }
    let records = collect::collect(&ordered(envelopes));
    assert_eq!(
        records.len(),
        5,
        "one assistant batch plus four tool responses"
    );
    let calls = records[0].tool_calls.as_ref().unwrap();
    assert_eq!(calls.len(), 4);
    for (call, response) in calls.iter().zip(&records[1..]) {
        assert_eq!(response.tool_call_id.as_deref(), Some(call.id.as_str()));
    }
}

#[test]
fn still_rejects_orphan_duplicate_and_response_before_request() {
    for envelopes in [
        vec![tool_request_envelope("request", "req-1", Some("turn"))],
        vec![tool_response_envelope("response", "req-1", Some("turn"))],
        vec![
            tool_response_envelope("response", "req-1", Some("turn")),
            tool_request_envelope("request", "req-1", Some("turn")),
        ],
        vec![
            tool_request_envelope("request-1", "req-1", Some("turn")),
            tool_request_envelope("request-2", "req-2", Some("turn")),
            tool_response_envelope("response-1", "req-1", Some("turn")),
            tool_response_envelope("response-2", "req-1", Some("turn")),
        ],
    ] {
        assert!(collect::collect(&ordered(envelopes)).is_empty());
    }
}
