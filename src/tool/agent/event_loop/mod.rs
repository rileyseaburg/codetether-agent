//! Event collection for sub-agent message runs.
//!
//! This module coordinates streaming session events and final handle
//! resolution for `message` actions.
//!
//! # Examples
//!
//! ```ignore
//! let result = run(&mut rx, handle).await;
//! ```

mod collect;
mod finalize;
mod state;

use crate::session::{Session, SessionEvent};
use anyhow::Result;
use serde_json::Value;
use tokio::sync::mpsc;

/// Collects streaming output and resolves the spawned task handle.
///
/// # Examples
///
/// ```ignore
/// let (response, thinking, tools, error, session) = run(&mut rx, handle).await;
/// ```
pub(super) async fn run(
    rx: &mut mpsc::Receiver<SessionEvent>,
    handle: tokio::task::JoinHandle<Result<Session>>,
) -> (String, String, Vec<Value>, Option<String>, Option<Session>) {
    let mut state = collect::collect_events(rx).await;
    let updated_session = finalize::finish_handle(handle, &mut state.error).await;
    (
        state.response,
        state.thinking,
        state.tools,
        state.error,
        updated_session,
    )
}
