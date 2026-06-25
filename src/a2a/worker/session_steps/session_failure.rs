//! Records a visible diagnostic into the session when a worker step fails or
//! the step budget is exhausted, so an autonomous run never ends wordlessly.

use crate::provider::{CompletionResponse, ContentPart, Message, Role};
use crate::session::Session;

/// Unwrap a worker step result, recording a halt diagnostic into the session
/// (and persisting it) before propagating any error, so a failed model call
/// never ends the run without leaving a visible reason.
pub async fn step_or_record(
    session: &mut Session,
    step: usize,
    result: anyhow::Result<CompletionResponse>,
) -> anyhow::Result<CompletionResponse> {
    match result {
        Ok(response) => Ok(response),
        Err(error) => {
            record_loop_halt(session, &format!("model step {step} failed: {error}")).await;
            Err(error)
        }
    }
}

/// Append a system-visible note explaining why the agent loop stopped, then
/// persist the session so the reason survives a silent worker death.
///
/// # Examples
///
/// ```ignore
/// // internal worker helper; called from run_session_steps
/// record_loop_halt(&mut session, "step budget (50) exhausted").await;
/// ```
pub async fn record_loop_halt(session: &mut Session, reason: &str) {
    let note = format!("⚠️ Agent loop halted: {reason}");
    session.add_message(Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text { text: note }],
    });
    if let Err(error) = session.save().await {
        tracing::error!(error = %error, "Failed to persist loop-halt diagnostic");
    }
}
