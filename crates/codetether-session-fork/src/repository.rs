//! Persistence boundary for edited-conversation branches.
use crate::{ForkError, ForkSession, service};

/// Load a source and save only the new branch; never write back the original session.
/// # Errors
/// Propagates target conflicts and durable initialization/storage failures.
/// # Arguments
/// * `id` — Source conversation identity.
/// * `index` — Expected user-message position.
/// * `expected` — Exact observed text.
/// # Returns
/// The new persisted branch; the original is never saved.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_fork::ForkSession>(id: &str) {
/// let _ = codetether_session_fork::create_persisted::<S>(id, 0, "expected").await;
/// # }
/// ```
pub async fn create<S: ForkSession>(
    id: &str,
    index: usize,
    expected: &str,
) -> Result<S, ForkError> {
    let source = S::load(id).await?;
    let branch = service::create(&source, index, expected).await?;
    branch.save().await?;
    Ok(branch)
}
