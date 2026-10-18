//! Prepare and launch one accepted message under current parent policy.
use crate::{Started, StartupHost};
use anyhow::Result;
use codetether_agent_input::Params;
use codetether_collaboration_input::MessageImage;
use codetether_collaboration_state::thread_status;
use tokio::sync::mpsc;
/// Start a message after policy preparation, registry loading and working-state effects.
/// # Arguments
/// * `agent_id` — Canonical child identity.
/// * `message` — Message text.
/// * `images` — Input images passed to the native image-capable prompt API.
/// * `params` — Trusted current parent context.
/// # Returns
/// The native event receiver and child session task.
/// # Errors
/// Preserves session-preparation and registry errors with tracked status.
/// # Examples
/// Use an application [StartupHost] implementation.
pub async fn begin_message<H: StartupHost>(
    agent_id: &str,
    message: String,
    images: Vec<MessageImage>,
    params: &Params,
) -> Result<Started<H::SourceEvent, H::Session>>
where
    H::SourceEvent: 'static,
{
    codetether_collaboration_state::residency::touch(agent_id);
    let session = thread_status::track_error(agent_id, H::message_session(agent_id, params).await)?;
    let registry = thread_status::track_error(agent_id, H::registry().await)?;
    H::open_steering(agent_id);
    thread_status::running(agent_id);
    H::announce_working(agent_id, "processing message");
    codetether_agent_trace::begin(agent_id, message.clone());
    let (tx, rx) = mpsc::channel::<H::SourceEvent>(256);
    let handle =
        tokio::spawn(async move { H::prompt(session, message, Some(images), tx, registry).await });
    Ok((rx, handle))
}
