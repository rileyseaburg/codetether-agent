use std::path::Path;

use anyhow::Result;

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::constants::FILE_SHARE_MAX_BYTES;
use crate::tui::models::{InputMode, ViewMode};

pub fn attach_file_to_input(app: &mut App, workspace_dir: &Path, path: &Path) {
    let file_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        workspace_dir.join(path)
    };

    if !file_path.exists() {
        push_system_message(
            app,
            format!("File not found: {}", file_path.display()),
            "File not found",
        );
        return;
    }

    if !file_path.is_file() {
        push_system_message(
            app,
            format!("Not a file: {}", file_path.display()),
            "Path is not a file",
        );
        return;
    }

    let display_path = display_path_for_workspace(&file_path, workspace_dir);
    match build_file_share_snippet(&file_path, &display_path, FILE_SHARE_MAX_BYTES) {
        Ok((snippet, truncated, binary)) => {
            if !app.state.input.trim().is_empty() {
                app.state.input.push_str("\n\n");
            }
            app.state.input.push_str(&snippet);
            app.state.input_cursor = app.state.input.chars().count();
            app.state.input_mode = InputMode::Editing;
            app.state.history_index = None;
            app.state.refresh_slash_suggestions();
            app.state.set_view_mode(ViewMode::Chat);
            app.state.scroll_to_bottom();

            let suffix = if binary {
                " (binary file metadata only)".to_string()
            } else if truncated {
                format!(" (truncated to {} bytes)", FILE_SHARE_MAX_BYTES)
            } else {
                String::new()
            };

            let message =
                format!("Attached `{display_path}` to composer{suffix}. Press Enter to send.");
            push_system_message(app, message, "Attached file to composer");
        }
        Err(err) => {
            push_system_message(
                app,
                format!("Failed to attach file {}: {err}", file_path.display()),
                "Failed to attach file",
            );
        }
    }
}

fn push_system_message(app: &mut App, message: String, status: &str) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, message));
    app.state.status = status.to_string();
    app.state.scroll_to_bottom();
}

fn display_path_for_workspace(path: &Path, workspace_dir: &Path) -> String {
    path.strip_prefix(workspace_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}

fn build_file_share_snippet(
    path: &Path,
    display_path: &str,
    max_bytes: usize,
) -> Result<(String, bool, bool)> {
    let bytes = std::fs::read(path)?;

    if bytes.contains(&0) {
        let snippet = format!(
            "Shared file: {display_path}\n[binary file, size: {}]",
            format_bytes(bytes.len() as u64)
        );
        return Ok((snippet, false, true));
    }

    let mut text = String::from_utf8_lossy(&bytes).to_string();
    let mut truncated = false;
    if text.len() > max_bytes {
        let mut end = max_bytes;
        while end > 0 && !text.is_char_boundary(end) {
            end -= 1;
        }
        text.truncate(end);
        truncated = true;
    }

    let language = path
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| !ext.is_empty())
        .unwrap_or("text");

    let mut snippet = format!("Shared file: {display_path}\n~~~{language}\n{text}\n~~~");
    if truncated {
        snippet.push_str(&format!(
            "\n[truncated to first {} bytes; original size: {}]",
            max_bytes,
            format_bytes(bytes.len() as u64)
        ));
    }

    Ok((snippet, truncated, false))
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if bytes < 1024 {
        format!("{bytes}B")
    } else if (bytes as f64) < MB {
        format!("{:.1}KB", bytes as f64 / KB)
    } else if (bytes as f64) < GB {
        format!("{:.1}MB", bytes as f64 / MB)
    } else {
        format!("{:.2}GB", bytes as f64 / GB)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_file_share_uses_metadata_only() {
        let dir = tempfile::tempdir().expect("tempdir should create");
        let path = dir.path().join("blob.bin");
        std::fs::write(&path, [0_u8, 1, 2, 3]).expect("binary fixture should write");

        let (snippet, truncated, binary) =
            build_file_share_snippet(&path, "blob.bin", FILE_SHARE_MAX_BYTES)
                .expect("snippet should build");

        assert!(binary);
        assert!(!truncated);
        assert!(snippet.contains("[binary file, size: 4B]"));
    }
}
