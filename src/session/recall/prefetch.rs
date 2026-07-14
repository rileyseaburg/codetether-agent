//! Prompt-time recall injection before the first provider request.

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

const PREFETCH_LIMIT: usize = 3;
const PREFETCH_MINIMUM_SCORE: f32 = 0.45;

pub(crate) async fn message(session: &Session) -> Option<Message> {
    if !crate::session::helper::runtime::prior_context_allowed_for_session(session) {
        return None;
    }
    let workspace = session.metadata.directory.as_deref()?;
    let query = crate::session::helper::text::latest_user_text(&session.messages)?;
    let hits = super::search::run(
        workspace,
        &query,
        super::search::SearchOptions {
            session_id: None,
            excluded_session: Some(&session.id),
            limit: PREFETCH_LIMIT,
            minimum_score: PREFETCH_MINIMUM_SCORE,
        },
    )
    .await
    .ok()?;
    if hits.is_empty() {
        return None;
    }
    Some(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: super::render::evidence(
                &hits,
                "Relevant prior-session evidence retrieved locally before this request. Use only when relevant and retain its provenance:",
            ),
        }],
    })
}
