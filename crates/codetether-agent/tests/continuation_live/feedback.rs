//! Resume a capture with explicitly reviewed tool observations supplied by the tester.
//! This module never executes proposed tools or shells.
use super::replay_support::Session;
use anyhow::Result;
use codetether_agent::provider::{Message, Role};
pub(super) fn apply(source: &mut Session, end: usize) -> Result<()> {
    let Ok(path) = std::env::var("CODETETHER_REPLAY_FEEDBACK") else {
        return Ok(());
    };
    let values: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
    let Some(value) = values.get(end.to_string()) else {
        return Ok(());
    };
    let messages: Vec<Message> = serde_json::from_value(value.clone())?;
    source.pages.truncate(source.messages.len());
    for message in messages {
        anyhow::ensure!(
            matches!(message.role, Role::Assistant | Role::Tool),
            "feedback must contain only observed assistant/tool turns"
        );
        source
            .pages
            .push(codetether_context_policy::pages::classify(&message));
        source.messages.push(message);
    }
    Ok(())
}
