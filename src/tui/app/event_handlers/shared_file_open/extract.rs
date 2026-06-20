//! Extract a file path from a chat message: shared `File` parts and the
//! `path` argument of file-oriented tool calls (`read`/`write`/`edit`/…).

use std::path::{Path, PathBuf};

use crate::tui::chat::message::{ChatMessage, MessageType};

/// File-tool names whose `path` argument identifies a referenced file.
const FILE_TOOLS: &[&str] = &[
    "read",
    "write",
    "edit",
    "multiedit",
    "fileinfo",
    "file_info",
    "apply_patch",
    "patch",
];

/// Resolve a file path referenced by `msg`, if any.
pub(super) fn path_from_message(msg: &ChatMessage, workspace_dir: &Path) -> Option<PathBuf> {
    match &msg.message_type {
        MessageType::File { path, .. } => Some(resolve(workspace_dir, path)),
        MessageType::ToolCall { name, arguments } if is_file_tool(name) => {
            path_from_args(arguments).map(|p| resolve(workspace_dir, &p))
        }
        _ => None,
    }
}

fn is_file_tool(name: &str) -> bool {
    FILE_TOOLS.contains(&name)
}

fn path_from_args(arguments: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(arguments).ok()?;
    for key in ["path", "file_path", "file"] {
        if let Some(p) = value.get(key).and_then(|v| v.as_str()) {
            return Some(p.to_string());
        }
    }
    None
}

fn resolve(workspace_dir: &Path, path: &str) -> PathBuf {
    let p = Path::new(path);
    if p.is_absolute() {
        p.to_path_buf()
    } else {
        workspace_dir.join(p)
    }
}
