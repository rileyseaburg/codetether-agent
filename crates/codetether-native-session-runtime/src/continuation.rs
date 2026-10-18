//! Native transcript and journal bindings for final-message continuation.
pub use codetether_session_orchestration::Continuation;
use codetether_session_orchestration::Session;
/// Drain accepted input or continue the current goal after a final message.
/// # Errors
/// Propagates goal-journal errors without treating them as completion.
pub async fn after_message(session: &mut impl Session) -> anyhow::Result<Continuation> {
    codetether_session_orchestration::after_message::<
        codetether_native_goals::runtime::journal::NativeJournal,
        _,
    >(session)
    .await
}
/// Continue an active goal using its existing journal and transcript.
/// # Errors
/// Propagates goal-journal errors.
pub async fn continue_goal(session: &mut impl Session) -> anyhow::Result<Continuation> {
    codetether_session_orchestration::continue_goal::<
        codetether_native_goals::runtime::journal::NativeJournal,
        _,
    >(session)
    .await
}
