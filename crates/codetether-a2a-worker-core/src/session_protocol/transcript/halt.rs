use super::Host;
use codetether_provider_core::{ContentPart, Message, Role};
/// Append a visible halt diagnostic and attempt to persist it.
/// # Arguments
/// `host` supplies transcript storage; `reason` describes the stop condition.
/// # Returns
/// Completes after saving, logging a persistence failure without propagating it.
/// # Examples
/// ```no_run
/// # async fn halt(host: &mut impl codetether_a2a_worker_core::session_protocol::transcript::Host) {
/// codetether_a2a_worker_core::session_protocol::transcript::record_loop_halt(host, "budget exhausted").await;
/// # }
/// ```
pub async fn record_loop_halt(host: &mut impl Host, reason: &str) {
    let note = format!("⚠️ Agent loop halted: {reason}");
    host.append(Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text { text: note }],
    });
    if let Err(error) = host.save().await {
        tracing::error!(error = %error, "Failed to persist loop-halt diagnostic");
    }
}
