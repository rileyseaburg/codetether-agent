//! Session-scoped input steering for active prompt runs.

mod drain;
mod guard;
mod input;
#[cfg(unix)]
mod ipc;
mod queue;

pub(crate) use drain::{drain_into, drain_or_close_into};
pub(crate) use guard::RunGuard;
pub(crate) use input::SteeringInput;
pub(crate) use queue::{clear, open, push};

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
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let accepted = codetether_agent::session::helper::steering::send(
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

#[cfg(test)]
mod tests;
