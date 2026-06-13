//! Owning-session wrapper over the keep-last compression core.

use std::sync::Arc;

use anyhow::Result;
use chrono::Utc;

use crate::session::Session;

use super::context::CompressContext;
use super::keep_last::compress_messages_keep_last;

/// Back-compat wrapper over [`compress_messages_keep_last`] that operates
/// on an owning [`Session`]. Bumps [`Session::updated_at`] when the
/// buffer was rewritten.
///
/// # Errors
///
/// Propagates any error from the underlying core.
pub async fn compress_history_keep_last(
    session: &mut Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    keep_last: usize,
    reason: &str,
) -> Result<bool> {
    let ctx = CompressContext::from_session(session);
    let did = compress_messages_keep_last(
        &mut session.messages,
        &ctx,
        provider,
        model,
        keep_last,
        reason,
    )
    .await?;
    if did {
        session.updated_at = Utc::now();
    }
    Ok(did)
}
