//! Records a visible diagnostic into the session when a worker step fails or
//! the step budget is exhausted, so an autonomous run never ends wordlessly.

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

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
