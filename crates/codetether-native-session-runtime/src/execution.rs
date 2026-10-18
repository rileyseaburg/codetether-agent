//! Native execution ownership with the application's durable goal gate.
/// Exclusive execution lease guarded by the native durable goal policy.
///
/// # Examples
/// ```no_run
/// let guard = codetether_native_session_runtime::RunGuard::open("session-id")?;
/// let _control = guard.handle();
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct RunGuard(codetether_session_control::RunGuard);
impl RunGuard {
    /// Return the shared control handle for this session lease.
    pub fn handle(&self) -> super::ControlHandle {
        self.0.handle()
    }
    /// Acquire execution ownership using the durable goal gate.
    /// # Errors
    /// Returns an error if ownership or the goal gate rejects the session.
    pub fn open(session_id: &str) -> anyhow::Result<Self> {
        codetether_session_control::RunGuard::open(session_id, super::control_gate::check).map(Self)
    }
    /// Run work within this lease and propagate its result.
    /// # Errors
    /// Returns execution-control or delegated-work errors.
    pub async fn run<T>(
        &mut self,
        future: impl std::future::Future<Output = anyhow::Result<T>>,
    ) -> anyhow::Result<T> {
        self.0.run(future).await
    }
}
