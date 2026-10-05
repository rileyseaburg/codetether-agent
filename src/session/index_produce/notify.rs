//! Session mutation notifications for proactive RLM preparation.

use crate::provider::Message;
use crate::session::Session;

pub(crate) fn appended(session: &mut Session, message: Message) {
    crate::session::store::evict::before_append(session);
    let appended_idx = session.messages.len();
    session.messages.push(message);
    session.summary_index.append(appended_idx);
}

pub(crate) fn saved(session: &Session) {
    super::proactive::schedule(session);
}

pub(crate) async fn removed(session_id: &str) {
    super::proactive::remove(session_id).await;
}
