//! Final handle resolution for sub-agent event loops.
//!
//! This module ensures the spawned message task is aborted or joined cleanly
//! after event streaming finishes.
//!
//! # Examples
//!
//! ```ignore
//! let session = finish_handle(handle, false, &mut error).await;
//! ```

use super::ChildTask;
use crate::session::Session;

/// Resolves the background task handle after streaming has finished.
///
/// # Examples
///
/// ```ignore
/// let session = finish_handle(handle, false, &mut error).await;
/// ```
pub(super) async fn finish_handle(
    handle: ChildTask,
    timed_out: bool,
    error: &mut Option<String>,
) -> Option<Session> {
    if timed_out {
        handle.abort();
        let _ = handle.join().await;
        return None;
    }
    match handle.join().await {
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
