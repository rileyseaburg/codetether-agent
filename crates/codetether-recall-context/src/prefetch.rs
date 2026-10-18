//! Prompt-time recall injection before the first provider request.

use codetether_provider_core::{ContentPart, Message, Role};

const PREFETCH_LIMIT: usize = 3;
const PREFETCH_MINIMUM_SCORE: f32 = 0.45;

/// Prepare automatic evidence from other sessions when prior context is allowed.
/// # Arguments
/// Receives the current session view and native recall query host.
/// # Returns
/// A system message with up to three relevant evidence hits, or None when
/// denied, missing query inputs, search fails, or no evidence is found.
/// # Examples
/// ```no_run
/// # async fn example(source: &impl codetether_recall_context::Source, host: &impl codetether_recall_query::Host) {
/// let evidence = codetether_recall_context::message(source, host).await;
/// # }
/// ```
pub async fn message<S: crate::Source, H: codetether_recall_query::Host>(
    session: &S,
    host: &H,
) -> Option<Message> {
    if !session.prior_context_allowed() {
        return None;
    }
    let workspace = session.workspace()?;
    let query = codetether_context_policy::transcript::latest_user_text(session.messages())?;
    let hits = codetether_recall_query::run(
        workspace,
        &query,
        codetether_recall_query::SearchOptions {
            session_id: None,
            excluded_session: Some(session.id()),
            limit: PREFETCH_LIMIT,
            minimum_score: PREFETCH_MINIMUM_SCORE,
        },
        host,
    )
    .await
    .ok()?;
    if hits.is_empty() {
        return None;
    }
    Some(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: codetether_session_recall::render::evidence(
                &hits,
                "Relevant prior-session evidence retrieved locally before this request. Use only when relevant and retain its provenance:",
            ),
        }],
    })
}
