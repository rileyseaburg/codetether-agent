//! Persistence of completed stream items carried by an exhausted retry error.

use crate::provider::{Message, Role};
use crate::session::Session;

pub(in crate::session::helper) async fn persist(
    session: &mut Session,
    error: &anyhow::Error,
) -> anyhow::Result<()> {
    let Some(content) = super::super::stream::checkpointed_content(error) else {
        return Ok(());
    };
    session.add_message(Message {
        role: Role::Assistant,
        content,
    });
    session.save().await
}
