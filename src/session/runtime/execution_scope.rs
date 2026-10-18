//! Same-session handoff using the application's durable execution gate.
pub(crate) struct ExecutionScope(codetether_session_control::ExecutionScope);
impl ExecutionScope {
    pub(crate) fn open(session_id: &str) -> anyhow::Result<Self> {
        codetether_session_control::ExecutionScope::open(session_id, super::control_gate::check)
            .map(Self)
    }
    pub(crate) async fn run<T>(
        &mut self,
        future: impl std::future::Future<Output = anyhow::Result<T>>,
    ) -> anyhow::Result<T> {
        self.0.run(future).await
    }
}
