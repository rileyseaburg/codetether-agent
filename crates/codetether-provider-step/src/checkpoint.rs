//! Persistence of confirmed stream items after retry exhaustion.
use anyhow::Result;
use codetether_provider_core::{Message, Role};
use std::future::Future;

/// Session storage operations needed to retain checkpointed assistant output.
/// # Implementors
/// Applications append to their existing transcript and persist it using the
/// same session storage implementation as successful turns.
pub trait Host {
    /// Append the recovered assistant message to the transcript.
    fn append(&mut self, message: Message);
    /// Persist the session after checkpoint content is appended.
    /// # Errors
    /// Returns errors from the application's session storage.
    fn save(&self) -> impl Future<Output = Result<()>> + Send;
}

/// Recover confirmed stream items from an error and save them as assistant output.
/// # Arguments
/// * `host` — Session transcript and persistence operations.
/// * `error` — Provider failure that may carry confirmed checkpoint content.
/// # Returns
/// Unit after persistence, or immediately when the error has no checkpoint.
/// # Errors
/// Propagates session persistence failures after appending the checkpoint.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_provider_step::checkpoint::Host, error: &anyhow::Error) -> anyhow::Result<()> {
/// codetether_provider_step::checkpoint::persist(host, error).await?;
/// # Ok(()) }
/// ```
pub async fn persist(host: &mut impl Host, error: &anyhow::Error) -> Result<()> {
    let Some(content) = codetether_stream_restart::checkpointed_content(error) else {
        return Ok(());
    };
    host.append(Message {
        role: Role::Assistant,
        content,
    });
    host.save().await
}
