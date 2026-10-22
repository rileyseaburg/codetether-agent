//! Bounded historical page reads and staged, revision-checked pin edits.
use crate::Snapshot;
use anyhow::{Result, ensure};
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;

/// One canonical historical message with an independently staged page classification.
/// # Examples
/// ```
/// use codetether_session_store::pin_edits::PinSource;
/// use codetether_context_policy::pages::PageKind;
/// use codetether_provider_core::{Message,Role};
/// let source = PinSource { index: 4, message: Message {role:Role::User,content:vec![]},
///     original: PageKind::Conversation, page: PageKind::Constraint };
/// assert_ne!(source.original, source.page);
/// ```
#[derive(Clone, Debug)]
pub struct PinSource {
    /// Absolute canonical message position.
    pub index: usize,
    /// Complete original message, never rewritten by page edits.
    pub message: Message,
    /// Page read at the acknowledged revision.
    pub original: PageKind,
    /// Requested replacement classification.
    pub page: PageKind,
}

/// Stage at most two historical sources without loading intervening messages.
/// # Arguments
/// `session` supplies an acknowledged checkpoint; `indices` are absolute positions.
/// # Returns
/// Unit after caching only sources outside the loaded window.
/// # Errors
/// Rejects stale revisions, missing records, read-only snapshots and dirty staged edits.
/// # Panics
/// Panics if the checkpoint mutex is poisoned or no Tokio runtime is available.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_store::Snapshot>(session: &S) -> anyhow::Result<()> {
/// codetether_session_store::pin_edits::prepare(session, &[0, 100]).await?;
/// # Ok(()) }
/// ```
pub async fn prepare<S: Snapshot>(session: &S, indices: &[usize]) -> Result<()> {
    ensure!(
        indices.len() <= 2,
        "at most two pin sources can be prepared"
    );
    let snapshot = session.state().0.lock().unwrap().clone();
    ensure!(
        !snapshot.read_only,
        "cannot edit a historical consumer snapshot"
    );
    ensure!(
        snapshot
            .pin_sources
            .iter()
            .all(|source| source.original == source.page),
        "save or discard pending pin edits before preparing more sources"
    );
    let end = snapshot.message_start + session.messages().len();
    let mut indices: Vec<_> = indices
        .iter()
        .copied()
        .filter(|index| !(snapshot.message_start..end).contains(index))
        .collect();
    indices.sort_unstable();
    indices.dedup();
    let revision = snapshot.revision;
    let sources = if indices.is_empty() {
        Vec::new()
    } else {
        ensure!(
            snapshot.id == session.id() && revision > 0,
            "historical sources require a persisted session"
        );
        tokio::task::spawn_blocking(move || super::pin_read::read(&snapshot, &indices)).await??
    };
    let mut checkpoint = session.state().0.lock().unwrap();
    ensure!(
        checkpoint.revision == revision,
        "SESSION_REVISION_CONFLICT: session changed during pin lookup"
    );
    ensure!(
        checkpoint
            .pin_sources
            .iter()
            .all(|source| source.original == source.page),
        "pending pin edits changed during source lookup"
    );
    checkpoint.pin_sources = sources;
    Ok(())
}
