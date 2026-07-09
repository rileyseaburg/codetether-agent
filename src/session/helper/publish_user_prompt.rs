//! Publish the user's prompt to the agent bus as a [`BusMessage::UserPrompt`].
//!
//! Called by `run_prompt` and `run_prompt_with_events` right after the user
//! message is appended to the session. This closes the SFT loop for the S3
//! training sink: tool-call traces now have a matching `user` role record
//! carrying workspace + session provenance.
//!
//! [`BusMessage::UserPrompt`]: crate::bus::BusMessage::UserPrompt

use crate::session::Session;

/// Publish a `UserPrompt` bus message for `message` if the session has a bus.
///
/// Non-fatal: when the session has no bus attached this is a no-op.
pub(crate) fn publish(session: &Session, message: &str) {
    let Some(ref bus) = session.bus else {
        return;
    };
    let workspace = session
        .metadata
        .directory
        .clone()
        .or_else(|| std::env::current_dir().ok())
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    let handle = bus.handle(&session.agent);
    handle.send(
        format!("agent.{}.user", session.agent),
        crate::bus::BusMessage::UserPrompt {
            agent_id: session.agent.clone(),
            text: message.to_string(),
            workspace,
            session_id: session.id.clone(),
        },
    );
}
