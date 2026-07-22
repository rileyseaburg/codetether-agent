//! Evidence checks for commands that address an existing process session.

use crate::provider::{ContentPart, Message, Role};
use anyhow::{Context, Result, bail};
use serde_json::Value;

pub(super) fn validate(calls: &[(String, String)], messages: &[Message]) -> Result<()> {
    let polls = calls
        .iter()
        .filter(|(name, _)| name == "write_stdin")
        .collect::<Vec<_>>();
    if polls.is_empty() {
        return Ok(());
    }
    if calls.iter().any(|(name, _)| name != "write_stdin") {
        bail!("write_stdin cannot share a batch with another tool; wait for real results first");
    }
    let evidence = trailing_tool_results(messages);
    for (_, arguments) in polls {
        let value: Value = serde_json::from_str(arguments)?;
        let id = value
            .get("session_id")
            .and_then(Value::as_u64)
            .context("write_stdin requires an integer session_id")?;
        let marker = format!("Script running with session ID {id}");
        if !evidence.iter().any(|result| result.contains(&marker)) {
            bail!("write_stdin session_id {id} is not present in trailing real tool results");
        }
    }
    Ok(())
}

fn trailing_tool_results(messages: &[Message]) -> Vec<&str> {
    messages
        .iter()
        .rev()
        .take_while(|message| message.role == Role::Tool)
        .flat_map(|message| &message.content)
        .filter_map(|part| match part {
            ContentPart::ToolResult { content, .. } => Some(content.as_str()),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
#[path = "session_guard_tests.rs"]
mod tests;
