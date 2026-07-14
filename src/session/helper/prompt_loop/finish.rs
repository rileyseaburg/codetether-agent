//! Persistence and result construction after the shared loop.

use super::Runner;
use crate::session::{SessionEvent, SessionResult};
use anyhow::Result;

/// Persists the session, archives events, and builds the final result.
///
/// # Errors
///
/// Returns an error when session persistence fails.
pub(super) async fn finish(runner: &mut Runner<'_>) -> Result<SessionResult> {
    runner.session.save().await?;
    super::super::archive::archive_event_stream_to_s3(
        &runner.session.id,
        super::super::archive::event_stream_path(),
    )
    .await;
    if let Some(tx) = &runner.events {
        let _ = tx.send(SessionEvent::Done).await;
    }
    let text =
        super::super::evidence::gate_final_answer(runner.progress.output.trim(), runner.session);
    Ok(SessionResult {
        text,
        session_id: runner.session.id.clone(),
    })
}
