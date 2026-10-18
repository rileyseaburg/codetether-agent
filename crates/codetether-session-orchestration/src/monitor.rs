//! Track delegated cleanup, elapsed usage and goal holds without dropping work.
use crate::{Session, drain};
use anyhow::Result;
use codetether_goal_runtime::Journal;
use codetether_session_control::ControlHandle;

/// Wait for delegated work while servicing steering, durable holds and accounting.
/// # Arguments
/// `session` is the canonical parent transcript; `control` is its run handle;
/// `future` is delegated work; `J` resolves the parent's goal journal.
/// # Returns
/// The delegated result unless a previously observed hold, cancellation or
/// accounting error takes precedence. Delegated cleanup is allowed to finish.
/// # Errors
/// Returns delegated errors, cancellation, or goal-journal/gate errors.
/// # Panics
/// Requires a Tokio runtime with time enabled; input-log poisoning may panic.
/// # Examples
/// ```rust,no_run
/// use codetether_session_orchestration::{Session, monitor};
/// use codetether_session_control::ControlHandle;
/// use codetether_goal_runtime::Journal;
/// # async fn example<J: Journal, S: Session>(session: &mut S, control: &ControlHandle) -> anyhow::Result<()> {
/// let value = monitor::<J, S, _>(session, control, async { Ok(7) }).await?;
/// assert_eq!(value, 7);
/// # Ok(()) }
/// ```
pub async fn monitor<J: Journal, S: Session, T>(
    session: &mut S,
    control: &ControlHandle,
    future: impl std::future::Future<Output = Result<T>>,
) -> Result<T> {
    tokio::pin!(future);
    let mut tick = tokio::time::interval(std::time::Duration::from_millis(100));
    let mut stopping = None;
    let started = std::time::Instant::now();
    loop {
        tokio::select! {
            result = &mut future => {
                let accounting = codetether_goal_runtime::record_usage::<J>(session.id(), 0, started.elapsed()).await;
                if stopping.is_none() { stopping = accounting.err(); }
                if stopping.is_none() {
                    stopping = control.ready().await.err();
                }
                while drain(session, control, stopping.is_some() || result.is_err()) > 0 {}
                return match stopping { Some(error) => Err(error), None => result };
            }
            _ = tick.tick(), if stopping.is_none() => {
                if let Err(error) = codetether_goal_runtime::ensure_running::<J>(session.id()).await {
                    stopping = Some(error);
                    control.cancel();
                } else { drain(session, control, false); }
            }
            _ = control.cancelled(), if stopping.is_none() => {
                stopping = Some(anyhow::anyhow!("Session execution cancelled"));
            }
        }
    }
}
