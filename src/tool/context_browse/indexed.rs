//! Bounded history browsing: page listings or fetch one absolute sequence.
use super::ContextBrowseAction;
use crate::session::{Session, history_files};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Value, json};
pub(super) async fn run(
    session: &Session,
    action: ContextBrowseAction,
    args: &Value,
) -> Result<ToolResult> {
    let (start, limit) = match action {
        ContextBrowseAction::ShowTurn { turn } => (turn, 1),
        ContextBrowseAction::ListTurns => (
            args["offset"].as_u64().unwrap_or(0) as usize,
            args["limit"].as_u64().unwrap_or(100).clamp(1, 512) as usize,
        ),
    };
    let messages = Session::read_messages(&session.id, start, limit).await?;
    let dir = history_files::history_dir_for_session(session)?;
    tokio::fs::create_dir_all(&dir).await?;
    let mut paths = Vec::new();
    for (index, message) in messages.iter().enumerate() {
        let path = history_files::format_turn_path(
            &dir,
            start + index,
            history_files::role_label(&message.role),
        );
        tokio::fs::write(&path, history_files::render_turn(message)).await?;
        paths.push(path);
    }
    let result = match action {
        ContextBrowseAction::ListTurns => ToolResult::success(super::render_listing(&paths))
            .with_metadata("next_offset", json!(start + messages.len()))
            .with_metadata("total", json!(session.message_count())),
        ContextBrowseAction::ShowTurn { turn } => match messages.first() {
            Some(message) => ToolResult::success(history_files::render_turn(message))
                .with_metadata("path", json!(paths[0].display().to_string())),
            None => ToolResult::error(format!(
                "turn {turn} out of range (have {} entries)",
                session.message_count()
            )),
        },
    };
    Ok(result
        .with_metadata("session_id", json!(session.id))
        .truncate_to(crate::tool::tool_output_budget()))
}
