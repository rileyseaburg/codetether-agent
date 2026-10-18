use super::{App, Arc, ProviderRegistry, SessionSlot, TuiSessionHandle};
use crate::{session::Session, tui::app::session_runtime::PromptRequest};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub slot: &'a mut SessionSlot,
    pub runtime: &'a TuiSessionHandle,
}
impl codetether_watchdog::recovery::Host for Native<'_> {
    type Ui = App;
    type Session = Session;
    type Request = PromptRequest;
    type Provider = Arc<ProviderRegistry>;
    fn ui(&mut self) -> &mut App {
        self.app
    }
    fn take_session(&mut self) -> Option<Session> {
        self.slot.take_for_prompt()
    }
    fn request(
        session: Session,
        prompt: String,
        provider: &Arc<ProviderRegistry>,
    ) -> PromptRequest {
        let original_dir = session.metadata.directory.clone();
        PromptRequest::new(
            session,
            prompt,
            Vec::new(),
            Arc::clone(provider),
            original_dir,
            None,
        )
    }
    async fn submit(&self, request: PromptRequest) -> Result<(), PromptRequest> {
        self.runtime.submit(request).await
    }
    fn restore(&mut self, request: PromptRequest) {
        self.slot.restore(request.session);
    }
}
