//! Frontend prompt, worktree and completion-notice execution bindings.
/// Declare the frontend specialization of owned prompt execution.
///
/// Concrete payload types and callbacks stay selected by the frontend. The
/// shared executor preserves cancellation, worktree finalization and notice order.
/// # Examples
/// See the frontend session-runtime module for concrete resource selection.
#[macro_export]
macro_rules! declare_frontend_execution {
    ($session:path, $registry:path, $image:path, $event:path, $notice:path,
     $worktree:path, $result:path, $request:path, $active:path;
     $create:path, $prompt:path, $notice_fn:path, $worktree_result:path $(,)?) => {
        mod execute_host {
            use ::std::{
                path::{Path, PathBuf},
                sync::Arc,
            };
            use ::tokio::sync::mpsc;
            use $event as SessionEvent;
            use $image as ImageAttachment;
            use $notice as SessionNotice;
            use $registry as ProviderRegistry;
            use $result as PromptRunResult;
            use $session as Session;
            pub(super) struct Native;
            impl $crate::execute::Host for Native {
                type Session = Session;
                type Registry = ProviderRegistry;
                type Image = ImageAttachment;
                type Worktree = $worktree;
                type Event = SessionEvent;
                type Notice = SessionNotice;
                type Error = anyhow::Error;
                async fn create_worktree(
                    cwd: &Path,
                    session: &mut Session,
                ) -> Option<Self::Worktree> {
                    $create(cwd, session).await
                }
                fn restore_directory(session: &mut Session, directory: Option<PathBuf>) {
                    session.metadata.directory = directory;
                }
                async fn prompt(
                    session: &mut Session,
                    prompt: &str,
                    images: Vec<ImageAttachment>,
                    events: mpsc::Sender<SessionEvent>,
                    registry: Arc<ProviderRegistry>,
                    original_dir: Option<PathBuf>,
                ) -> anyhow::Result<()> {
                    $prompt(session, prompt, images, events, registry, original_dir).await
                }
                fn notice(result: PromptRunResult, session: Session) -> SessionNotice {
                    $notice_fn(result, session)
                }
                fn finished(notice: &SessionNotice) -> bool {
                    matches!(notice, SessionNotice::Finished(_))
                }
                async fn worktree_result(
                    success: bool,
                    worktree: Option<Self::Worktree>,
                    prompt: Option<&str>,
                ) {
                    $worktree_result(success, worktree, prompt).await;
                }
            }
        }
        mod execute {
            use ::std::sync::Arc;
            use ::tokio::sync::{Notify, mpsc};
            use $active as ActiveCancel;
            use $event as SessionEvent;
            use $notice as SessionNotice;
            use $request as PromptRequest;
            pub(super) async fn run(
                request: PromptRequest,
                event_tx: mpsc::Sender<SessionEvent>,
                notice_tx: mpsc::Sender<SessionNotice>,
                cancel: Arc<Notify>,
                active: ActiveCancel,
            ) {
                $crate::execute::run::<super::execute_host::Native, _>(
                    request, event_tx, notice_tx, cancel, active,
                )
                .await;
            }
        }
    };
}
