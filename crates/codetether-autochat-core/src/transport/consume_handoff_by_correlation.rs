//! Correlated relay consume handoff by correlation.
use super::*;

/// Wait for a correlated text handoff addressed to the recipient.
///
/// # Arguments
/// * `receivers` - Registered recipient subscriptions.
/// * `agent_id` - Recipient identity.
/// * `correlation_id` - Expected handoff correlation.
///
/// # Returns
/// The first matching text part. Unrelated queued messages are consumed.
///
/// # Errors
/// Returns an error for an unregistered recipient, bus closure, missing text or the five-second timeout.
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// use std::collections::HashMap;
/// use codetether_autochat_core::transport::consume_handoff_by_correlation;
/// let mut receivers = HashMap::new();
/// assert!(consume_handoff_by_correlation(&mut receivers, "missing", "turn").await.is_err());
/// # Ok(())
/// # }
/// ```
pub async fn consume_handoff_by_correlation(
    receivers: &mut HashMap<String, BusHandle>,
    agent_id: &str,
    correlation_id: &str,
) -> Result<String> {
    let receiver = receivers
        .get_mut(agent_id)
        .ok_or_else(|| anyhow!("No relay receiver registered for @{agent_id}"))?;

    let wait = async {
        loop {
            let envelope = receiver
                .recv_mine()
                .await
                .ok_or_else(|| anyhow!("Bus closed while waiting for @{agent_id} handoff"))?;

            if envelope.correlation_id.as_deref() != Some(correlation_id) {
                continue;
            }

            let BusMessage::AgentMessage { parts, .. } = envelope.message else {
                continue;
            };
            if let Some(text) = first_text_part(&parts) {
                return Ok(text);
            }

            return Err(anyhow!(
                "Received relay handoff for @{agent_id} without text payload"
            ));
        }
    };

    timeout(Duration::from_secs(HANDOFF_TIMEOUT_SECS), wait)
        .await
        .context("Timed out waiting for correlated relay handoff")?
}
