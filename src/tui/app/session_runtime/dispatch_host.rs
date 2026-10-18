//! Native review gates, notices and executor spawning.
use super::{PromptRequest, SessionNotice, active_cancel::ActiveCancel};
use crate::session::SessionEvent;
use std::sync::Arc;
use tokio::sync::{Notify, mpsc};
pub(super) struct Native {
    pub cancel: ActiveCancel,
    pub event_tx: mpsc::Sender<SessionEvent>,
    pub notice_tx: mpsc::Sender<SessionNotice>,
}
impl codetether_session_runtime::dispatch::Host<PromptRequest> for Native {
    fn held(&self, request: &PromptRequest) -> bool {
        crate::session::tasks::runtime::answer_review::held(&request.session.id)
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
        tokio::spawn(super::execute::run(
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
