//! Native transcript and journal bindings for final-message continuation.
use crate::session::Session;
pub(crate) use codetether_session_orchestration::Continuation;
pub(crate) async fn after_message(session: &mut Session) -> anyhow::Result<Continuation> {
    codetether_session_orchestration::after_message::<super::goals::journal::NativeJournal, _>(
        session,
    )
    .await
}
pub(crate) async fn continue_goal(session: &mut Session) -> anyhow::Result<Continuation> {
    codetether_session_orchestration::continue_goal::<super::goals::journal::NativeJournal, _>(
        session,
    )
    .await
}
