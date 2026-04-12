//! Post-prompt worktree cleanup and result handling.
//!
//! Runs the prompt against the provider, then delegates
//! merge/PR logic to [`super::merge`] and cleans up.
//!
//! # Examples
//!
//! ```ignore
//! handle_worktree_result(&result, worktree_state).await;
//! run_prompt(&mut session, "hi", images, tx, reg, dir).await;
//! ```

use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session, SessionEvent};

use super::merge::push_or_merge;
use super::worktree::WorktreeState;

/// Run the prompt against the provider and restore the
/// original directory on completion.
///
/// # Examples
///
/// ```ignore
/// let r = run_prompt(&mut s, "hi", vec![], tx, reg, dir).await;
/// ```
pub(super) async fn run_prompt(
    session: &mut Session,
    prompt: &str,
    images: Vec<ImageAttachment>,
    event_tx: mpsc::Sender<SessionEvent>,
    registry: Arc<ProviderRegistry>,
    original_dir: Option<PathBuf>,
) -> anyhow::Result<Session> {
    session
        .prompt_with_events_and_images(prompt, images, event_tx, registry)
        .await
        .map(|_| {
            session.metadata.directory = original_dir;
            session.clone()
        })
}

/// Handle worktree merge/PR and cleanup after prompt.
///
/// Pushes a PR on success, falls back to local merge on PR
/// failure, and always cleans up the worktree.
///
/// # Examples
///
/// ```ignore
/// handle_worktree_result(&Ok(session), Some((mgr, wt))).await;
/// ```
pub(super) async fn handle_worktree_result(
    result: &anyhow::Result<Session>,
    worktree: Option<WorktreeState>,
) {
    let Some((mgr, wt, base_branch)) = worktree else {
        return;
    };
    if result.is_ok() {
        push_or_merge(&mgr, &wt, base_branch.as_deref()).await;
    }
    if let Err(e) = mgr.cleanup(&wt.name).await {
        tracing::warn!(error = %e, "Failed to cleanup TUI worktree");
    }
}
