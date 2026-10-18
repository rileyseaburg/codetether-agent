//! Native borrowed-history binding for shared snapshot serialization.
use super::Session;
use anyhow::Result;
#[cfg(test)]
#[path = "persistence_snapshot_tests.rs"]
mod tests;
pub(super) fn serialize(session: &Session) -> Result<Vec<u8>> {
    codetether_session_codec::serialize(codetether_session_codec::SnapshotView {
        header: super::codec::header(session),
        messages: &session.messages,
        pages: &session.pages,
        summary_index: &session.summary_index,
        tool_uses: &session.tool_uses,
    })
}
