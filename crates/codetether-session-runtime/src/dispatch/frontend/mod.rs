//! Frontend dispatcher state, startup and guarded executor submission.
mod submit;
/// Declare frontend runtime startup and dispatch-host bindings.
///
/// The shared dispatcher owns command ordering; selected callbacks preserve the
/// review gate and executor, and the selected cancellation type owns run cleanup.
/// # Examples
/// See the frontend session-runtime module for concrete type and callback paths.
#[macro_export]
macro_rules! declare_frontend_runtime_dispatch {
    ($request:path, $notice:path, $event:path, $active:path, $handle:path;
     $held:path, $execute:path $(,)?) => {
        mod dispatch_host {
            use ::std::sync::Arc;
            use ::tokio::sync::{Notify, mpsc};
            use $active as ActiveCancel;
            use $event as SessionEvent;
            use $notice as SessionNotice;
            use $request as PromptRequest;
            pub(super) struct Native {
                pub cancel: ActiveCancel,
                pub event_tx: mpsc::Sender<SessionEvent>,
                pub notice_tx: mpsc::Sender<SessionNotice>,
            }
            impl $crate::dispatch::Host<PromptRequest> for Native {
                fn held(&self, request: &PromptRequest) -> bool {
                    $held(&request.session.id)
                }
                fn release_prepared(&self, request: &PromptRequest) {
                    self.cancel.release_prepared(&request.session.id);
                }
                fn attach(&self, request: &PromptRequest, notify: Arc<Notify>) -> bool {
                    self.cancel.set(&request.session.id, notify)
                }
                async fn reject(&mut self, request: PromptRequest, error: &'static str) {
                    let _ = self
                        .notice_tx
                        .send(SessionNotice::Failed {
                            session: request.session,
                            error: error.into(),
                        })
                        .await;
                }
                async fn started(&mut self) {
                    let _ = self.notice_tx.send(SessionNotice::Started).await;
                }
                fn spawn(&self, request: PromptRequest, notify: Arc<Notify>) {
                    tokio::spawn($execute(
                        request,
                        self.event_tx.clone(),
                        self.notice_tx.clone(),
                        notify,
                        self.cancel.clone(),
                    ));
                }
                fn cancel(&self) {
                    self.cancel.notify();
                }
            }
        }
        mod runtime {
            use ::tokio::sync::mpsc;
            use $event as SessionEvent;
            use $handle as TuiSessionHandle;
            use $notice as SessionNotice;
            /// Spawn the TUI-owned session runtime task.
            ///
            /// The returned [`TuiSessionHandle`] sends commands to the spawned task. The
            /// task continues until it receives a shutdown command or its command channel is
            /// closed.
            ///
            /// # Arguments
            ///
            /// * `event_tx` - Channel used by prompt execution to stream [`SessionEvent`]
            ///   values back to the TUI.
            /// * `notice_tx` - Channel used by the runtime to report [`SessionNotice`]
            ///   lifecycle updates such as prompt start, completion, or failure.
            ///
            /// # Returns
            ///
            /// A handle connected to the runtime command channel.
            ///
            /// # Side Effects
            ///
            /// Spawns a Tokio task and allocates an internal command channel with capacity
            /// for eight queued commands.
            pub(crate) fn spawn(
                event_tx: mpsc::Sender<SessionEvent>,
                notice_tx: mpsc::Sender<SessionNotice>,
            ) -> TuiSessionHandle {
                $crate::dispatch::spawn(|cancel| super::dispatch_host::Native {
                    cancel,
                    event_tx,
                    notice_tx,
                })
            }
        }
    };
}
