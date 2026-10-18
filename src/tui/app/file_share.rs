use std::path::Path;

use codetether_file_share::{build_file_share_snippet, display_path_for_workspace};

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
            app.state.history.history_index = None;
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
