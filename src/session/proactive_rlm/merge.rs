//! Prepared-index validation and merge into an immutable session snapshot.

use crate::session::Session;

use super::{freshness, store};

pub(crate) async fn prepared_index(session: &Session) -> crate::session::index::SummaryIndex {
    let Some(prepared) = store::read(session).await else {
        return crate::session::index::SummaryIndex::new();
    };
    let valid = freshness::matches(
        &session.messages,
        prepared.message_count,
        prepared.fingerprint,
    );
    if valid {
        prepared.index
    } else {
        crate::session::index::SummaryIndex::new()
    }
}
