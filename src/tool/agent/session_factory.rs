//! Session construction helpers for spawned agents.
//!
//! This module creates initialized sub-agent sessions with the correct
//! system prompt and model metadata. It keeps session bootstrapping
//! separate from spawn validation and storage.
//!
//! # Examples
//!
//! ```ignore
//! let session = create_agent_session("reviewer", "Audit the code", model).await?;
//! ```

use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use anyhow::{Context, Result};

/// Creates a fresh session for a spawned sub-agent.
///
/// # Examples
///
/// ```ignore
/// let session = create_agent_session("reviewer", "Audit the code", "glm-5").await?;
/// ```
pub(super) async fn create_agent_session(
    name: &str,
    instructions: &str,
    model: &str,
) -> Result<Session> {
    let mut session = Session::new().await.context("Failed to create session")?;
    session.set_agent_name(name.to_string());
    session.metadata.model = Some(model.to_string());
    session.add_message(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: build_system_message(name, instructions),
        }],
    });
    Ok(session)
}

fn build_system_message(name: &str, instructions: &str) -> String {
    format!(
        "You are @{name}, a specialized sub-agent. {instructions}\n\n\
         You have access to all tools. Be thorough, focused, and concise. Complete the task fully."
    )
}
