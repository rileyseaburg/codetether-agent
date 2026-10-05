//! Map durable global summary ranges into the current working window.
use crate::session::{
    Session,
    index::{SummaryIndex, SummaryRange},
};
pub(super) async fn read(session: &Session) -> Option<SummaryIndex> {
    let value = crate::session::store::projection::read(&session.id, "rlm")
        .await
        .ok()??;
    let mut index = SummaryIndex::new();
    let offset = session.message_offset();
    for doc in value["documents"].as_array()? {
        let start = doc["start"].as_u64()? as usize;
        let end = doc["end"].as_u64()? as usize;
        if start < offset || end > offset + session.messages.len() {
            continue;
        }
        let node = serde_json::from_value(doc["node"].clone()).ok()?;
        index.insert(SummaryRange::new(start - offset, end - offset)?, node);
    }
    Some(index)
}
