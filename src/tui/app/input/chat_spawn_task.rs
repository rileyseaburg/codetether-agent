//! Tokio task body for the provider spawn.
//!
//! Contains the unwinding wrapper and panic recovery used
//! by [`super::chat_spawn::spawn_provider_task`].

use super::worktree::WorktreeState;
use super::worktree_result::{handle_worktree_result, run_prompt};
use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session, SessionEvent};
use futures::FutureExt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{Notify, mpsc};

/// Run the provider inside a panic-catching wrapper.
///
/// Delegates to [`run_prompt`] then handles worktree result
/// and sends the outcome through `result_tx`.
///
/// `cancel` is a shared [`Notify`] that, when triggered (e.g. by the TUI
/// when the user submits a steering message mid-stream), aborts the
/// in-flight provider call so the partial assistant content already
/// streamed via `event_tx` can be treated as a completed turn and the
/// new user message dispatched immediately.
pub(super) async fn run_spawned_task(
    mut session: Session,
    prompt: String,
    images: Vec<ImageAttachment>,
    event_tx: mpsc::Sender<SessionEvent>,
    registry: Arc<ProviderRegistry>,
    original_dir: Option<PathBuf>,
    result_tx: mpsc::Sender<anyhow::Result<Session>>,
    worktree: Option<WorktreeState>,
    prompt_for_pr: String,
    cancel: Arc<Notify>,
) {
    // Clone so we can restore `metadata.directory` on the cancel branch
    // without racing with `run_prompt`, which also consumes `original_dir`.
    let original_dir_for_cancel = original_dir.clone();
    let result = std::panic::AssertUnwindSafe(async {
        tokio::select! {
            biased;
            _ = cancel.notified() => {
                session.metadata.directory = original_dir_for_cancel;
                Ok::<(), anyhow::Error>(())
            }
            r = run_prompt(
                &mut session,
                &prompt,
                images,
                event_tx,
                registry,
                original_dir,
            ) => r,
        }?;
        Ok(session)
    })
    .catch_unwind()
    .await;
    match result {
        Ok(inner) => {
            handle_worktree_result(&inner, worktree, Some(&prompt_for_pr)).await;
            let _ = result_tx.send(inner).await;
        }
        Err(panic_err) => {
            let msg = extract_panic_message(&panic_err);
            tracing::error!(error = %msg, "Provider task panicked");
            let _ = result_tx
                .send(Err(anyhow::anyhow!("Provider task panicked: {msg}")))
                .await;
        }
    }
}

/// Extract a readable message from a panic payload.
fn extract_panic_message(err: &Box<dyn std::any::Any + Send>) -> &str {
    err.downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| err.downcast_ref::<&str>().copied())
        .unwrap_or("unknown panic")
}
