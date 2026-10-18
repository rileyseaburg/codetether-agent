//! Worker swarm result persistence and response construction.
use super::{Session, SessionResult};
use codetether_a2a_worker_core::session_protocol::{policy, transcript};
use codetether_native_providers::{ContentPart, Message, Role};
use codetether_swarm_core::SwarmResult;
pub(super) async fn apply(
    session: &mut impl Session,
    result: SwarmResult,
) -> anyhow::Result<(SessionResult, bool)> {
    let text =
        codetether_swarm_core::worker_result::swarm_result_text(&result.result, result.success);
    transcript::Host::append(
        session,
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text { text: text.clone() }],
        },
    );
    transcript::Host::save(session).await?;
    Ok((
        SessionResult {
            text,
            session_id: policy::Source::session_id(session).to_owned(),
        },
        result.success,
    ))
}
