//! Prompt execution for the TUI session runtime.
//!
//! This module owns the background task body for a single submitted prompt. It
//! moves the prompt request into the executor, runs the prompt against its
//! session, supports cooperative cancellation, converts success, error, or panic
//! outcomes into a [`SessionNotice`], and reports prompt completion back to the
//! runtime loop.

use std::sync::Arc;

use futures::FutureExt;
use tokio::sync::{Notify, mpsc};

use crate::session::SessionEvent;
use crate::tui::app::input::worktree_result::{handle_worktree_result, run_prompt};

use super::{PromptRequest, SessionNotice};

/// Run one moved-session prompt and return the session by notice.
///
/// The session runtime spawns this function for each accepted prompt request.
/// It consumes the [`PromptRequest`], streams prompt events through `event_tx`,
/// and sends exactly one lifecycle notice through `notice_tx` after the prompt
/// finishes, fails, is cancelled, or panics. A final message is also sent on
/// `done_tx` so the runtime loop can clear its active cancellation state.
///
/// Cancellation is cooperative. If `cancel` is notified before `run_prompt`
/// completes, the session directory is restored to the original directory and
/// the prompt is treated as a successful cancellation path for notice creation.
/// Panics inside the selected async block are caught and converted by
/// `super::prompt_result::notice`.
///
/// # Arguments
///
/// * `request` - Prompt request containing the moved session, prompt text,
///   images, tool registry, original directory, worktree metadata, and
///   pull-request prompt metadata.
/// * `event_tx` - Channel used to stream [`SessionEvent`] values produced while
///   the prompt runs.
/// * `notice_tx` - Channel used to return the final [`SessionNotice`] with the
///   session state.
/// * `cancel` - Shared notifier used by the runtime loop to request
///   cancellation of this prompt.
/// * `done_tx` - Channel used to tell the runtime loop that this executor task
///   has reached its cleanup point.
///
/// # Side Effects
///
/// Runs provider/tool work through `run_prompt`, may update session metadata,
/// may perform worktree result handling through `handle_worktree_result`, sends
/// a final notice, and signals task completion. Send failures are ignored
/// because they only indicate that the receiving runtime or UI has gone away.
pub(super) async fn run(
    request: PromptRequest,
    event_tx: mpsc::Sender<SessionEvent>,
    notice_tx: mpsc::Sender<SessionNotice>,
    cancel: Arc<Notify>,
    done_tx: mpsc::Sender<()>,
) {
    let PromptRequest {
        mut session,
        prompt,
        images,
        registry,
        original_dir,
        worktree_root,
        prompt_for_pr,
    } = request;
    let cancel_dir = original_dir.clone();
    let worktree = match worktree_root {
        Some(cwd) => crate::tui::app::input::worktree::create_worktree(&cwd, &mut session).await,
        None => None,
    };
    let result = std::panic::AssertUnwindSafe(async {
        tokio::select! {
            biased;
            _ = cancel.notified() => {
                session.metadata.directory = cancel_dir;
                Ok(())
            }
            r = run_prompt(&mut session, &prompt, images, event_tx, registry, original_dir) => r,
        }
    })
    .catch_unwind()
    .await;
    let notice = super::prompt_result::notice(result, session);
    handle_worktree_result(
        matches!(notice, SessionNotice::Finished(_)),
        worktree,
        Some(&prompt_for_pr),
    )
    .await;
    let _ = notice_tx.send(notice).await;
    let _ = done_tx.send(()).await;
}
