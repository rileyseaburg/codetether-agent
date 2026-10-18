//! Forward only textual parts of an inter-agent message.
use codetether_a2a_core::Part;
pub(crate) fn payload(worker_id: &str, from: &str, to: &str, parts: &[Part]) -> serde_json::Value {
    let text = parts
        .iter()
        .filter_map(|p| {
            if let Part::Text { text } = p {
                Some(text.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    serde_json::json!({
        "type": "agent_message",
        "worker_id": worker_id,
        "from": from,
        "to": to,
        "text": text,
    })
}
