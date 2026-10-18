//! Same-session handoff using the application's durable execution gate.
/// Same-session handoff scope retaining the native durable execution gate.
pub struct ExecutionScope(codetether_session_control::ExecutionScope);
impl ExecutionScope {
    /// Enter a handoff scope for a session already owned by the current task.
    /// # Errors
    /// Returns ownership or durable-goal gate errors.
    pub fn open(session_id: &str) -> anyhow::Result<Self> {
        codetether_session_control::ExecutionScope::open(session_id, super::control_gate::check)
            .map(Self)
    }
    /// Run delegated work within the handoff scope.
    /// # Errors
    /// Returns execution-control or delegated-work errors.
    pub async fn run<T>(
        &mut self,
        future: impl std::future::Future<Output = anyhow::Result<T>>,
    ) -> anyhow::Result<T> {
        self.0.run(future).await
    }
}
