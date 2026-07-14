//! Worker swarm result persistence and response construction.

use crate::provider::{ContentPart, Message, Role};
use crate::session::{Session, SessionResult};
use crate::swarm::SwarmResult;
use anyhow::Result;

pub(super) async fn apply(
    session: &mut Session,
    result: SwarmResult,
) -> Result<(SessionResult, bool)> {
    let text = super::super::swarm_policy_result::swarm_result_text(&result.result, result.success);
    session.add_message(Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text { text: text.clone() }],
    });
    session.save().await?;
    Ok((
        SessionResult {
            text,
            session_id: session.id.clone(),
        },
        result.success,
    ))
}
