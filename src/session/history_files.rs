//! Materialized turn files for filesystem-style history browsing.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tokio::fs;

use crate::provider::{ContentPart, Message, Role};

use super::Session;

pub(crate) fn role_label(role: &Role) -> &'static str {
    match role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    }
}

pub(crate) fn render_turn(msg: &Message) -> String {
    let mut buf = String::new();
    for part in &msg.content {
        if !buf.is_empty() {
            buf.push_str("\n\n");
        }
        match part {
            ContentPart::Text { text } => buf.push_str(text),
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => {
                buf.push_str(&format!("[tool_result tool_call_id={tool_call_id}]\n"));
                buf.push_str(content);
            }
            ContentPart::ToolCall {
                name, arguments, ..
            } => buf.push_str(&format!("[tool_call {name}]\n{arguments}")),
            ContentPart::Image { url, .. } => buf.push_str(&format!("[image {url}]")),
            ContentPart::File { path, .. } => buf.push_str(&format!("[file {path}]")),
            ContentPart::Thinking { text } => buf.push_str(&format!("[thinking]\n{text}")),
        }
    }
    buf
}

pub(crate) fn format_turn_path(session_dir: &Path, turn: usize, role: &str) -> PathBuf {
    session_dir.join(format!("turn-{turn:04}-{role}.md"))
}

pub(crate) fn history_dir_for_session(session: &Session) -> Result<PathBuf> {
    let data_dir = if let Ok(explicit) = std::env::var("CODETETHER_DATA_DIR") {
        let explicit = explicit.trim();
        if !explicit.is_empty() {
            PathBuf::from(explicit)
        } else {
            workspace_data_dir(session.metadata.directory.as_deref())
        }
    } else {
        workspace_data_dir(session.metadata.directory.as_deref())
    };
    Ok(data_dir.join("history").join(&session.id))
}

pub(crate) async fn materialize_session_history(session: &Session) -> Result<Vec<PathBuf>> {
    let dir = history_dir_for_session(session)?;
    match fs::remove_dir_all(&dir).await {
        Ok(()) => {}
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => return Err(err).with_context(|| format!("remove {}", dir.display())),
    }
    fs::create_dir_all(&dir)
        .await
        .with_context(|| format!("create {}", dir.display()))?;

    let mut paths = Vec::with_capacity(session.history().len());
    for (idx, msg) in session.history().iter().enumerate() {
        let path = format_turn_path(&dir, idx, role_label(&msg.role));
        fs::write(&path, render_turn(msg))
            .await
            .with_context(|| format!("write {}", path.display()))?;
        paths.push(path);
    }
    Ok(paths)
}

fn workspace_data_dir(workspace: Option<&Path>) -> PathBuf {
    workspace
        .map(|start| {
            start
                .ancestors()
                .find(|path| path.join(".git").exists())
                .unwrap_or(start)
                .join(".codetether-agent")
        })
        .or_else(crate::config::Config::data_dir)
        .unwrap_or_else(|| PathBuf::from(".codetether-agent"))
}
