//! Session mutation notifications for proactive RLM preparation.

use crate::session::Session;

pub(crate) fn saved(session: &Session) {
    super::proactive::schedule(session);
}

pub(crate) async fn removed(session_id: &str) {
    super::proactive::remove(session_id).await;
}
