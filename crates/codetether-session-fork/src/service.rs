//! Build an independent prefix branch without modifying the original conversation.
use crate::ForkSession;
use codetether_provider_core::{ContentPart, Role};

use crate::ForkError;

/// Construct a fresh identity with context strictly before the selected user message.
/// # Errors
/// Returns Conflict for stale/non-user targets, or Storage if initialization fails.
/// # Arguments
/// * `source` — Original conversation.
/// * `index` — Expected user-message position.
/// * `expected` — Exact observed text.
/// # Returns
/// A fresh unsaved branch containing only earlier messages.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_fork::ForkSession>(source: &S) {
/// let _ = codetether_session_fork::create(source, 0, "expected").await;
/// # }
/// ```
pub async fn create<S: ForkSession>(
    source: &S,
    index: usize,
    expected: &str,
) -> Result<S, ForkError> {
    let Some(target) = source.messages().get(index) else {
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
    let mut branch = S::new().await?;
    branch.set_title(Some(format!(
        "{} (edited)",
        source.title().unwrap_or("Conversation")
    )));
    branch.set_metadata(crate::metadata::branch_metadata(
        source.metadata(),
        branch.id(),
        source.agent(),
    ));
    branch.set_agent_name(source.agent());
    for message in source.messages().iter().take(index) {
        branch.add_message(message.clone());
    }
    Ok(branch)
}
