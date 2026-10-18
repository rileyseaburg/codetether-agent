//! Session-scoped input steering for active prompt runs.
//!
//! [SteeringInput] preserves user text and images. [open] and [push] share one
//! activity-aware inbox registry. Implement [Transcript] to use [drain_into] and
//! [drain_or_close_into]. On Unix, `Endpoint` accepts [send] from other processes.

mod cleanup;
mod drain;
mod input;
#[cfg(unix)]
mod ipc;
mod queue;
mod transcript;

pub use cleanup::clear;
pub use drain::{drain_into, drain_or_close_into};
pub use input::SteeringInput;
#[cfg(unix)]
pub use ipc::Endpoint;
pub use queue::{open, push};

/// Send trusted user text to whichever process currently owns a session turn.
///
/// # Arguments
///
/// * `session_id` — Durable CodeTether session identifier.
/// * `text` — User message to inject at the next safe steering boundary.
///
/// # Returns
///
/// `true` when an active owner accepted the message, or `false` when idle.
///
/// # Errors
///
/// Returns an error for invalid input or local IPC failures.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
/// let accepted = codetether_session_steering::send(
///     "session-1234",
///     "Use the safer migration path",
/// ).await.unwrap();
/// # let _ = accepted;
/// # });
/// ```
pub async fn send(session_id: &str, text: &str) -> anyhow::Result<bool> {
    #[cfg(unix)]
    return ipc::send(session_id, text).await;
    #[cfg(not(unix))]
    {
        let _ = (session_id, text);
        Ok(false)
    }
}

pub use transcript::Transcript;

/// Process-wide follow-up prompt queued for a later turn.
pub mod follow_up;

/// Implementation support for application-local session binding.
#[doc(hidden)]
pub mod session_binding;
