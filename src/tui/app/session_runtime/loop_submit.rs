//! Submit prompt work from the runtime command loop.
//!
//! This module contains the handoff point between the command loop and the
//! asynchronous prompt executor. It accepts a [`PromptRequest`], records the
//! cancellation handle for the in-flight run, notifies observers that work has
//! started, and spawns the executor task that will stream session events and
//! completion notices.

use std::sync::Arc;

use tokio::sync::{Notify, mpsc};

use crate::session::SessionEvent;

use super::{PromptRequest, SessionNotice};

/// Submits a prompt request for asynchronous execution.
///
/// The command loop calls this when it receives a new prompt for a session. If
/// another prompt is already running, the request is rejected and a
/// [`SessionNotice::Failed`] notice is sent with a busy error. Otherwise, this
/// function creates a cancellation [`Notify`] handle, stores it in `cancel`,
/// emits [`SessionNotice::Started`], and spawns `super::execute::run` to process
/// the prompt in the background.
///
/// # Arguments
///
/// * `request` - Prompt and session metadata to execute.
/// * `cancel` - Slot for the active cancellation notifier. `Some` means the
///   runtime is already busy; `None` means this call may start a new run.
/// * `event_tx` - Channel used by the executor to publish streamed
///   [`SessionEvent`] values.
/// * `notice_tx` - Channel used to report lifecycle notices such as start,
///   failure, and completion.
/// * `done_tx` - Channel used by the executor to signal that the spawned run has
///   finished.
///
/// # Returns
///
/// Always returns `false`, indicating that submitting work does not by itself
/// terminate the surrounding command loop.
///
/// # Side Effects
///
/// Sends lifecycle notices on `notice_tx`, mutates `cancel` when a request is
/// accepted, and spawns a Tokio task for accepted work. Channel send failures are
/// intentionally ignored because they only mean the receiver has gone away.
///
/// # Preconditions
///
/// The caller must clear `cancel` when the corresponding executor task reports
/// completion; otherwise later submissions will continue to be treated as busy.
pub(super) async fn submit(
    request: PromptRequest,
    cancel: &mut Option<Arc<Notify>>,
    event_tx: &mpsc::Sender<SessionEvent>,
    notice_tx: &mpsc::Sender<SessionNotice>,
    done_tx: &mpsc::Sender<()>,
) -> bool {
    if cancel.is_some() {
        let _ = notice_tx
            .send(SessionNotice::Failed {
                session: request.session,
                error: "Session runtime is busy".into(),
            })
            .await;
        return false;
    }
    let notify = Arc::new(Notify::new());
    *cancel = Some(Arc::clone(&notify));
    let _ = notice_tx.send(SessionNotice::Started).await;
    tokio::spawn(super::execute::run(
        request,
        event_tx.clone(),
        notice_tx.clone(),
        notify,
        done_tx.clone(),
    ));
    false
}
