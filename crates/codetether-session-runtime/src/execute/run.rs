use super::Host;
use crate::{handle::Control, request::PromptRequest};
use futures::FutureExt;
use std::sync::Arc;
use tokio::sync::{Notify, mpsc};
/// Execute an owned prompt with cancellation and guaranteed active cleanup.
/// # Arguments
/// `request` owns inputs, `event_tx` streams events, `notice_tx` returns the
/// session, `cancel` interrupts prompt polling and `active` clears ownership.
/// # Returns
/// Completes after best-effort final notice delivery. Prompt errors and caught
/// panics are converted to notices; native pre/post hooks retain their behavior.
/// # Examples
/// ```no_run
/// # async fn execute<H: codetether_session_runtime::execute::Host, C: codetether_session_runtime::handle::Control + Send>(request: codetether_session_runtime::request::PromptRequest<H::Session, H::Registry, H::Image>, events: tokio::sync::mpsc::Sender<H::Event>, notices: tokio::sync::mpsc::Sender<H::Notice>, cancel: std::sync::Arc<tokio::sync::Notify>, active: C) {
/// codetether_session_runtime::execute::run::<H, C>(request, events, notices, cancel, active).await;
/// # }
/// ```
pub async fn run<H: Host, C: Control + Send>(
    request: PromptRequest<H::Session, H::Registry, H::Image>,
    event_tx: mpsc::Sender<H::Event>,
    notice_tx: mpsc::Sender<H::Notice>,
    cancel: Arc<Notify>,
    active: C,
) {
    let completion = super::guard::Guard::new(active);
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
        Some(cwd) => H::create_worktree(&cwd, &mut session).await,
        None => None,
    };
    let result = std::panic::AssertUnwindSafe(async {
        tokio::select! {
            biased;
            _ = cancel.notified() => {
                H::restore_directory(&mut session, cancel_dir);
                Ok(())
            }
            r = H::prompt(&mut session, &prompt, images, event_tx, registry, original_dir) => r,
        }
    })
    .catch_unwind()
    .await;
    let notice = H::notice(result, session);
    H::worktree_result(H::finished(&notice), worktree, Some(&prompt_for_pr)).await;
    completion.clear();
    let _ = notice_tx.send(notice).await;
}
