//! Provider task spawning with optional worktree isolation.
//!
//! Creates an isolated git worktree (when enabled), clones
//! the session, and spawns a Tokio task that runs the prompt
//! against the provider.  On completion it pushes a PR or
//! recovers with a local merge, then cleans up.

use std::path::Path;
use std::sync::Arc;

use futures::FutureExt;
use tokio::sync::mpsc;

use super::worktree::create_worktree;
use super::worktree_result::{handle_worktree_result, run_prompt};
use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session, SessionEvent};
use crate::tui::app::state::App;

/// Spawn the provider task with optional worktree isolation.
///
/// Sets up the worktree (if `use_worktree` is enabled),
/// clones the session, and spawns a background task that
/// calls the provider and handles the result.
pub(super) async fn spawn_provider_task(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Arc<ProviderRegistry>,
    prompt: &str,
    pending_images: Vec<ImageAttachment>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    session.metadata.auto_apply_edits = app.state.auto_apply_edits;
    let mut session_for_task = session.clone();
    let event_tx = event_tx.clone();
    let result_tx = result_tx.clone();
    let registry = Arc::clone(registry);
    let prompt = prompt.to_string();
    let use_worktree = app.state.use_worktree;

    let worktree_state = if use_worktree {
        create_worktree(app, cwd, &mut session_for_task).await
    } else {
        None
    };

    let original_dir = session.metadata.directory.clone();
    let prompt_for_pr = prompt.clone();

    tokio::spawn(async move {
        let result = std::panic::AssertUnwindSafe(run_prompt(
            &mut session_for_task,
            &prompt,
            pending_images,
            event_tx,
            registry,
            original_dir,
        ))
        .catch_unwind()
        .await;
        match result {
            Ok(ref r) => {
                handle_worktree_result(r, worktree_state, Some(&prompt_for_pr)).await;
                let _ = result_tx.send(result.unwrap()).await;
            }
            Err(panic_err) => {
                let msg = panic_err
                    .downcast_ref::<String>()
                    .map(|s| s.as_str())
                    .or_else(|| panic_err.downcast_ref::<&str>().copied())
                    .unwrap_or("unknown panic");
                tracing::error!(error = %msg, "Provider task panicked");
                let _ = result_tx
                    .send(Err(anyhow::anyhow!("Provider task panicked: {msg}")))
                    .await;
            }
        }
    });
}
