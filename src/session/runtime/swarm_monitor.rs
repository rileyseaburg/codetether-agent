//! Native transcript and journal bindings for delegated work monitoring.
use super::ControlHandle;
use crate::session::Session;
pub(crate) async fn run<T>(
    session: &mut Session,
    control: &ControlHandle,
    future: impl std::future::Future<Output = anyhow::Result<T>>,
) -> anyhow::Result<T> {
    codetether_session_orchestration::monitor::<super::goals::journal::NativeJournal, _, T>(
        session, control, future,
    )
    .await
}
