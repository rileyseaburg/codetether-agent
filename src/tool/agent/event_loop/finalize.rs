//! Final handle resolution for sub-agent event loops.
//!
//! This module ensures the spawned message task is aborted or joined cleanly
//! after event streaming finishes.
//!
//! # Examples
//!
//! ```ignore
//! let session = finish_handle(handle, &mut error).await;
//! ```

use crate::session::Session;
use anyhow::Result;

/// Resolves the background task handle after streaming has finished.
///
/// # Examples
///
/// ```ignore
/// let session = finish_handle(handle, &mut error).await;
/// ```
pub(super) async fn finish_handle(
    handle: tokio::task::JoinHandle<Result<Session>>,
    error: &mut Option<String>,
) -> Option<Session> {
    if !handle.is_finished() {
        handle.abort();
        return None;
    }
    match handle.await {
        Ok(Ok(session)) => Some(session),
        Ok(Err(err)) => {
            *error = Some(err.to_string());
            None
        }
        Err(err) => {
            *error = Some(err.to_string());
            None
        }
    }
}
