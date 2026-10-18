//! Project durable global summary ranges into the loaded transcript window.
use codetether_context_policy::summary_index::{SummaryIndex, SummaryRange};
use serde_json::Value;
/// Map durable global summary ranges into a loaded message window.
/// # Arguments
/// * `value` — Stored projection containing summary documents.
/// * `offset` — Absolute offset of the loaded window.
/// * `message_count` — Number of loaded messages.
/// # Returns
/// A window-relative index, or None if a document is malformed.
pub fn window_index(value: &Value, offset: usize, message_count: usize) -> Option<SummaryIndex> {
    let mut index = SummaryIndex::new();
    for doc in value["documents"].as_array()? {
        let start = doc["start"].as_u64()? as usize;
        let end = doc["end"].as_u64()? as usize;
        if start < offset || end > offset + message_count {
            continue;
        }
        let node = serde_json::from_value(doc["node"].clone()).ok()?;
        index.insert(SummaryRange::new(start - offset, end - offset)?, node);
    }
    Some(index)
}
