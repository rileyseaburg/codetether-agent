//! Transport helpers for protocol-first relay handoff consumption.

use crate::a2a::types::Part;
use crate::bus::{AgentBus, BusHandle, BusMessage};
use anyhow::{Context, Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration, timeout};

const HANDOFF_TIMEOUT_SECS: u64 = 5;

pub fn attach_handoff_receiver(
    receivers: &mut HashMap<String, BusHandle>,
    bus: Arc<AgentBus>,
    agent_id: &str,
) {
    if receivers.contains_key(agent_id) {
        return;
    }
    receivers.insert(agent_id.to_string(), bus.handle(agent_id.to_string()));
}

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

fn first_text_part(parts: &[Part]) -> Option<String> {
    parts.iter().find_map(|part| match part {
        Part::Text { text } => Some(text.clone()),
        _ => None,
    })
}
