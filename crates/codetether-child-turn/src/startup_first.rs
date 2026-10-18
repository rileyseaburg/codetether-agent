//! Prepare and launch the first prompt of a durable child.
use crate::{Started, StartupHost};
use anyhow::{Context, Result};
use codetether_collaboration_state::thread_status;
use tokio::sync::mpsc;
/// Start the assigned task after live lookup, registry loading and working-state effects.
/// # Arguments
/// * `agent_id` — Child identity to start.
/// # Returns
/// The native event receiver and child session task.
/// # Errors
/// Preserves missing-child and provider-registry errors with tracked status.
/// # Examples
/// Use an application [StartupHost] implementation.
pub async fn begin_first<H: StartupHost>(
    agent_id: &str,
) -> Result<Started<H::SourceEvent, H::Session>>
where
    H::SourceEvent: 'static,
{
    let (mut session, instructions) = thread_status::track_error(
        agent_id,
        H::first_session(agent_id).context(format!("Agent {agent_id} vanished after spawn")),
    )?;
    let kickoff = format!("Assigned task:\n\n{}", instructions);
    let registry = thread_status::track_error(agent_id, H::registry().await)?;
    H::ensure_bus(&mut session);
    H::open_steering(agent_id);
    thread_status::running(agent_id);
    H::announce_working(agent_id, "starting first turn");
    codetether_agent_trace::begin(agent_id, kickoff.clone());
    let (tx, rx) = mpsc::channel::<H::SourceEvent>(256);
    let handle = tokio::spawn(async move { H::prompt(session, kickoff, None, tx, registry).await });
    Ok((rx, handle))
}
