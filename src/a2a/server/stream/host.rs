//! Bind shared stream execution to application sessions and providers.
use super::*;
use crate::{provider::ProviderRegistry, session::Session};
use async_trait::async_trait;
use codetether_a2a_http::task_state::stream::driver::StreamSessionHost;
pub(super) struct Host<'a> {
    pub tasks: &'a Arc<DashMap<String, Task>>,
    pub bus: &'a Option<Arc<crate::bus::AgentBus>>,
    pub task_id: &'a String,
}
#[async_trait]
impl StreamSessionHost for Host<'_> {
    type Session = Session;
    type Registry = Arc<ProviderRegistry>;
    type Event = SessionEvent;
    async fn resolve(&mut self, context_id: Option<&str>) -> Result<Session> {
        crate::a2a::session_resolve::resolve_session(context_id).await
    }
    async fn configure(&mut self, session: &mut Session) {
        crate::a2a::session_config::configure(session).await;
    }
    fn spawn_events(&self, receiver: mpsc::Receiver<SessionEvent>) {
        events::spawn(
            self.bus,
            self.tasks,
            self.task_id,
            receiver,
            event_data::data,
        );
    }
    async fn registry(&mut self) -> Option<Arc<ProviderRegistry>> {
        registry::load(self.tasks, self.task_id).await
    }
    async fn execute(
        &mut self,
        session: &mut Session,
        prompt: &str,
        sender: mpsc::Sender<SessionEvent>,
        registry: Arc<ProviderRegistry>,
    ) -> Result<String> {
        session
            .prompt_with_events(prompt, sender, registry)
            .await
            .map(|result| result.text)
    }
    async fn persist(&mut self, session: &mut Session) {
        crate::a2a::session_config::persist(session).await;
    }
}
