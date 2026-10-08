//! Build an independent prefix branch without modifying the original conversation.
use crate::provider::{ContentPart, Role};
use crate::session::Session;

pub(super) use super::errors::ForkError;

/// Construct a fresh identity with context strictly before the selected user message.
/// # Errors
/// Returns Conflict for stale/non-user targets, or Storage if initialization fails.
pub(super) async fn create(
    source: &Session,
    index: usize,
    expected: &str,
) -> Result<Session, ForkError> {
    let Some(target) = source.messages.get(index) else {
        return Err(ForkError::Conflict);
    };
    let text = target
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    if target.role != Role::User || text != expected {
        return Err(ForkError::Conflict);
    }
    let mut branch = Session::new().await?;
    branch.title = Some(format!(
        "{} (edited)",
        source.title.as_deref().unwrap_or("Conversation")
    ));
    branch.metadata = source.metadata.clone();
    branch.metadata.run_checkpoint = None;
    branch.metadata.prior_context_turn_allowed = None;
    branch.metadata.knowledge_snapshot = None;
    branch.metadata.shared = false;
    branch.metadata.share_url = None;
    branch.metadata.delegation = Default::default();
    branch.metadata.provenance = Some(crate::provenance::ExecutionProvenance::for_session(
        &branch.id,
        &source.agent,
    ));
    branch.set_agent_name(&source.agent);
    for message in source.messages.iter().take(index) {
        branch.add_message(message.clone());
    }
    Ok(branch)
}
