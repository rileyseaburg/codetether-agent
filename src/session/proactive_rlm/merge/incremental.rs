//! Native projection read followed by shared window decoding.
use crate::session::{Session, index::SummaryIndex};
pub(super) async fn read(session: &Session) -> Option<SummaryIndex> {
    let value = crate::session::store::projection::read(&session.id, "rlm")
        .await
        .ok()??;
    codetether_prepared_context::window_index(
        &value,
        session.message_offset(),
        session.messages.len(),
    )
}
