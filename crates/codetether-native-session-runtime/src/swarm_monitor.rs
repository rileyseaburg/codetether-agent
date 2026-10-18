//! Native transcript and journal bindings for delegated work monitoring.
use super::ControlHandle;
use codetether_session_orchestration::Session;
/// Monitor delegated work while servicing the session control and goal state.
/// # Errors
/// Returns delegated-work, cancellation or journal errors after cleanup.
pub async fn run<T>(
    session: &mut impl Session,
    control: &ControlHandle,
    future: impl std::future::Future<Output = anyhow::Result<T>>,
) -> anyhow::Result<T> {
    codetether_session_orchestration::monitor::<
        codetether_native_goals::runtime::journal::NativeJournal,
        _,
        T,
    >(session, control, future)
    .await
}
