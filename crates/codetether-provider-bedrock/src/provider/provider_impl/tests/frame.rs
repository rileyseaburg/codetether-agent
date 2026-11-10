//! Match the AWS frame layout used by the protocol crate's decoding fixtures.
use serde_json::Value;

pub(super) fn event(kind: &str, payload: Value) -> Vec<u8> {
    let payload = serde_json::to_vec(&payload).unwrap();
    let mut headers = Vec::new();
    for (key, value) in [(":message-type", "event"), (":event-type", kind)] {
        headers.push(key.len() as u8);
        headers.extend_from_slice(key.as_bytes());
        headers.push(7);
        headers.extend_from_slice(&(value.len() as u16).to_be_bytes());
        headers.extend_from_slice(value.as_bytes());
    }
    let mut frame = Vec::new();
    frame.extend_from_slice(&((16 + headers.len() + payload.len()) as u32).to_be_bytes());
    frame.extend_from_slice(&(headers.len() as u32).to_be_bytes());
    // This decoder ignores CRCs, matching its existing unit fixtures.
    frame.extend_from_slice(&0u32.to_be_bytes());
    frame.extend(headers);
    frame.extend(payload);
    frame.extend_from_slice(&0u32.to_be_bytes());
    frame
}

pub(super) fn tool_start(index: u64, id: &str) -> Vec<u8> {
    event(
        "contentBlockStart",
        serde_json::json!({"contentBlockIndex":index,"start":{"toolUse":{"toolUseId":id,"name":"lookup"}}}),
    )
}

pub(super) fn tool_delta(index: u64, input: &str) -> Vec<u8> {
    event(
        "contentBlockDelta",
        serde_json::json!({"contentBlockIndex":index,"delta":{"toolUse":{"input":input}}}),
    )
}

pub(super) fn tool_stop(index: u64) -> Vec<u8> {
    event(
        "contentBlockStop",
        serde_json::json!({"contentBlockIndex":index}),
    )
}
