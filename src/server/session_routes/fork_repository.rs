//! Persistence boundary for edited-conversation branches.
use super::service::{self, ForkError};
use crate::session::Session;

/// Load a source and save only the new branch; never write back the original session.
/// # Errors
/// Propagates target conflicts and durable initialization/storage failures.
pub(super) async fn create(id: &str, index: usize, expected: &str) -> Result<Session, ForkError> {
    let source = Session::load(id).await?;
    let branch = service::create(&source, index, expected).await?;
    branch.save().await?;
    Ok(branch)
}
