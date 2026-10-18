use super::Host;
use codetether_session_events::{SessionEvent, SessionResult};
/// Persist and finalize a completed prompt run.
/// # Arguments
/// `host` supplies session storage, archival, events and evidence gating.
/// # Returns
/// The gated final answer and session identifier after persistence.
/// # Errors
/// Propagates session-save failures before archival or completion notification.
/// # Examples
/// ```no_run
/// # async fn finish(host: &mut impl codetether_prompt_loop::lifecycle::finish::Host) -> anyhow::Result<()> {
/// let _result = codetether_prompt_loop::lifecycle::finish::run(host).await?;
/// # Ok(()) }
/// ```
pub async fn run(host: &mut impl Host) -> anyhow::Result<SessionResult> {
    host.save().await?;
    host.archive().await;
    if let Some(tx) = host.events() {
        let _ = tx.send(SessionEvent::Done).await;
    }
    let text = host.final_text();
    Ok(SessionResult {
        text,
        session_id: host.session_id().to_owned(),
    })
}
