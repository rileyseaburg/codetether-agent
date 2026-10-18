//! Native execution ownership with the application's durable goal gate.
pub(crate) struct RunGuard(codetether_session_control::RunGuard);
impl RunGuard {
    pub(crate) fn handle(&self) -> super::ControlHandle {
        self.0.handle()
    }
    pub(crate) fn open(session_id: &str) -> anyhow::Result<Self> {
        codetether_session_control::RunGuard::open(session_id, super::control_gate::check).map(Self)
    }
    pub(crate) async fn run<T>(
        &mut self,
        future: impl std::future::Future<Output = anyhow::Result<T>>,
    ) -> anyhow::Result<T> {
        self.0.run(future).await
    }
}
