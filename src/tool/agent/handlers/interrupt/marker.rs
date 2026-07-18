//! Durable Codex-compatible marker for an intentionally interrupted turn.

use crate::provider::{ContentPart, Message, Role};
use anyhow::{Context, Result};

pub(super) const TEXT: &str = "The previous turn was interrupted on purpose. Any running unified exec processes may still be running in the background. If any tools/commands were aborted, they may have partially executed.";

pub(super) async fn persist(agent_id: &str) -> Result<()> {
    let entry = super::super::super::store::get(agent_id)
        .context("interrupted child disappeared before history persistence")?;
    let mut session = entry.session;
    session.add_message(Message {
        role: Role::Developer,
        content: vec![ContentPart::Text { text: TEXT.into() }],
    });
    session.save().await?;
    super::super::super::store::update_session(agent_id, session);
    Ok(())
}
