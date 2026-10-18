//! Native human-message bookkeeping and durable session writes.
use crate::{provider::Message, session::Session};
pub(super) fn add_human_message(session: &mut Session, message: Message) {
    session.add_human_message(message);
}
pub(super) async fn save(session: &Session) -> anyhow::Result<()> {
    session.save().await
}
