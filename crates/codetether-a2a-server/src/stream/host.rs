//! Bind shared stream execution to application sessions and providers.
use super::*;
use async_trait::async_trait;
use codetether_a2a_http::task_state::stream::driver::StreamSessionHost;
pub(super) struct Host<'a, H: Application> {
    pub application: PhantomData<fn() -> H>,
    pub tasks: &'a Arc<DashMap<String, Task>>,
    pub bus: &'a Option<Arc<codetether_bus_core::AgentBus>>,
    pub task_id: &'a String,
}
#[async_trait]
impl<H: Application> StreamSessionHost for Host<'_, H> {
    type Session = H::Session;
    type Registry = H::Registry;
    type Event = SessionEvent<H::Session>;
    async fn resolve(&mut self, context_id: Option<&str>) -> Result<H::Session> {
        H::resolve(context_id).await
    }
    async fn configure(&mut self, session: &mut H::Session) {
        H::configure(session).await;
    }
    fn spawn_events(&self, receiver: mpsc::Receiver<SessionEvent<H::Session>>) {
        events::spawn(
            self.bus,
            self.tasks,
            self.task_id,
            receiver,
            event_data::data::<H::Session>,
        );
    }
    async fn registry(&mut self) -> Option<H::Registry> {
        super::load_registry::<H>(self.tasks, self.task_id).await
    }
    async fn execute(
        &mut self,
        session: &mut H::Session,
        prompt: &str,
        sender: mpsc::Sender<SessionEvent<H::Session>>,
        registry: H::Registry,
    ) -> Result<String> {
        H::execute(session, prompt, sender, registry).await
    }
    async fn persist(&mut self, session: &mut H::Session) {
        H::persist(session).await;
    }
}
