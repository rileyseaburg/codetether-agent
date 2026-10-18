use super::{Arc, Native, ProviderRegistry};
use crate::{session::Session, tui::app::session_runtime::PromptRequest};
use codetether_stream_recovery::retry::{Host, State};
impl Host for Native<'_> {
    type Session = Session;
    type Request = PromptRequest;
    type Provider = Arc<ProviderRegistry>;
    fn state(&mut self) -> State<'_> {
        Native::state(self)
    }
    fn take_session(&mut self) -> Option<Session> {
        self.slot.take_for_prompt()
    }
    fn banner(&mut self) {
        Native::banner(self);
    }
    async fn wait(&self, duration: std::time::Duration) {
        tokio::time::sleep(duration).await;
    }
    fn begin_timing(&mut self) {
        Native::begin_timing(self);
    }
    fn request(
        session: Session,
        prompt: String,
        provider: &Arc<ProviderRegistry>,
    ) -> PromptRequest {
        let dir = session.metadata.directory.clone();
        PromptRequest::new(session, prompt, Vec::new(), Arc::clone(provider), dir, None)
    }
    async fn submit(&self, request: PromptRequest) -> Result<(), PromptRequest> {
        self.runtime.submit(request).await
    }
    fn restore(&mut self, request: PromptRequest) {
        self.slot.restore(request.session);
    }
}
