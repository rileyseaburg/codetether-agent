use std::path::Path;

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::image::is_image_file;

pub fn attach_path(app: &mut App, path: &Path) {
    if is_image_file(path) {
        attach_image(app, path);
    } else {
        let workspace = app.state.file_picker.workspace_dir.clone();
        crate::tui::app::file_share::attach_file_to_input(app, &workspace, path);
    }
    app.state.file_picker.active = false;
    app.state.view_mode = crate::tui::models::ViewMode::Chat;
}

fn attach_image(app: &mut App, path: &Path) {
    match crate::tui::app::input::attach_image_file(path) {
        Ok(attachment) => {
            app.state.pending_images.push(attachment);
            app.state.status = format!("Attached image {}.", path.display());
            app.state.messages.push(ChatMessage::new(
                MessageType::System,
                format!(
                    "Image attached: {}. Type a message and press Enter.",
                    path.display()
                ),
            ));
        }
        Err(msg) => {
            app.state.status = "Failed to attach image".to_string();
            app.state
                .messages
                .push(ChatMessage::new(MessageType::Error, msg));
        }
    }
    app.state.scroll_to_bottom();
}
