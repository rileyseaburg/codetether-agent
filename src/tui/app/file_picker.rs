//! File picker UI for browsing and attaching files (including images).
//!
//! Supports:
//! - Navigating directories
//! - Selecting text files (attached as code snippets)
//! - Selecting image files (attached as base64 image attachments)

use std::path::{Path, PathBuf};

use crate::session::ImageAttachment;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

/// A single entry in the file picker listing.
#[derive(Debug, Clone)]
pub struct FilePickerEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub name: String,
}

/// Supported image file extensions.
const IMAGE_EXTENSIONS: &[&str] = &["png", "jpg", "jpeg", "gif", "webp", "bmp", "svg"];

/// Check if a file path has an image extension.
pub fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .is_some_and(|ext| IMAGE_EXTENSIONS.contains(&ext.as_str()))
}

/// Open the file picker at the given working directory.
pub fn open_file_picker(app: &mut App, cwd: &Path) {
    let dir = cwd.to_path_buf();
    let entries = scan_directory(&dir);
    app.state.file_picker_active = true;
    app.state.file_picker_dir = dir;
    app.state.file_picker_entries = entries;
    app.state.file_picker_selected = 0;
    app.state.file_picker_filter.clear();
    app.state.view_mode = crate::tui::models::ViewMode::FilePicker;
    app.state.status = "File picker — Enter to select, Esc to cancel".to_string();
}

/// Handle Enter key in file picker — navigate into directories or select files.
pub fn file_picker_enter(app: &mut App, cwd: &Path) {
    let entry = app
        .state
        .file_picker_entries
        .get(app.state.file_picker_selected)
        .cloned();

    let Some(entry) = entry else {
        return;
    };

    if entry.is_dir {
        // Navigate into directory
        let entries = scan_directory(&entry.path);
        app.state.file_picker_dir = entry.path;
        app.state.file_picker_entries = entries;
        app.state.file_picker_selected = 0;
        app.state.file_picker_filter.clear();
        return;
    }

    // File selected — attach it
    let path = entry.path;
    if is_image_file(&path) {
        attach_image_from_picker(app, cwd, &path);
    } else {
        crate::tui::app::file_share::attach_file_to_input(app, cwd, &path);
    }

    app.state.file_picker_active = false;
    app.state.view_mode = crate::tui::models::ViewMode::Chat;
}

/// Handle Backspace in the filter input.
pub fn file_picker_filter_backspace(app: &mut App) {
    app.state.file_picker_filter.pop();
    rescan_with_filter(app);
}

/// Push a character to the filter.
pub fn file_picker_filter_push(app: &mut App, c: char) {
    app.state.file_picker_filter.push(c);
    rescan_with_filter(app);
}

/// Scan the current directory and populate entries.
fn scan_directory(dir: &Path) -> Vec<FilePickerEntry> {
    let mut entries: Vec<FilePickerEntry> = Vec::new();

    // Add parent directory entry (..) if not at root
    if dir.parent().is_some() {
        entries.push(FilePickerEntry {
            path: dir.parent().unwrap().to_path_buf(),
            is_dir: true,
            name: "..".to_string(),
        });
    }

    let Ok(read_dir) = std::fs::read_dir(dir) else {
        return entries;
    };

    let mut dir_entries: Vec<FilePickerEntry> = Vec::new();
    let mut file_entries: Vec<FilePickerEntry> = Vec::new();

    for entry in read_dir.flatten() {
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("?")
            .to_string();

        // Skip hidden files/dirs
        if name.starts_with('.') {
            continue;
        }

        if path.is_dir() {
            dir_entries.push(FilePickerEntry {
                path,
                is_dir: true,
                name: format!("{name}/"),
            });
        } else {
            let indicator = if is_image_file(&path) {
                " 📷"
            } else {
                ""
            };
            file_entries.push(FilePickerEntry {
                path,
                is_dir: false,
                name: format!("{name}{indicator}"),
            });
        }
    }

    // Sort: directories first, then files, alphabetically
    dir_entries.sort_by(|a, b| a.name.cmp(&b.name));
    file_entries.sort_by(|a, b| a.name.cmp(&b.name));

    entries.extend(dir_entries);
    entries.extend(file_entries);
    entries
}

/// Re-scan with current filter applied.
fn rescan_with_filter(app: &mut App) {
    let filter = app.state.file_picker_filter.to_ascii_lowercase();
    let all_entries = scan_directory(&app.state.file_picker_dir);

    if filter.is_empty() {
        app.state.file_picker_entries = all_entries;
    } else {
        app.state.file_picker_entries = all_entries
            .into_iter()
            .filter(|e| {
                e.name.to_ascii_lowercase().contains(&filter)
                    || e.is_dir && e.name == "../"
            })
            .collect();
    }
    app.state.file_picker_selected = 0;
}

/// Attach an image file from the file picker.
fn attach_image_from_picker(app: &mut App, _cwd: &Path, path: &Path) {
    match crate::tui::app::input::attach_image_file(path) {
        Ok(attachment) => {
            let display = path.display();
            app.state.pending_images.push(attachment);
            let count = app.state.pending_images.len();
            app.state.status = format!(
                "📷 Attached {display}. {count} image(s) pending. Press Enter to send."
            );
            app.state.messages.push(ChatMessage::new(
                MessageType::System,
                format!("📷 Image attached: {display}. Type a message and press Enter to send."),
            ));
            app.state.scroll_to_bottom();
        }
        Err(msg) => {
            app.state.messages.push(ChatMessage::new(
                MessageType::Error,
                format!("Failed to attach image: {msg}"),
            ));
            app.state.status = "Failed to attach image".to_string();
            app.state.scroll_to_bottom();
        }
    }
}

/// Select previous entry.
pub fn file_picker_select_prev(app: &mut App) {
    if app.state.file_picker_selected > 0 {
        app.state.file_picker_selected -= 1;
    }
}

/// Select next entry.
pub fn file_picker_select_next(app: &mut App) {
    if app.state.file_picker_selected + 1 < app.state.file_picker_entries.len() {
        app.state.file_picker_selected += 1;
    }
}

/// Render the file picker view.
pub fn render_file_picker(f: &mut ratatui::Frame, area: ratatui::prelude::Rect, app: &App) {
    use ratatui::prelude::*;
    use ratatui::widgets::*;

    let dir = app.state.file_picker_dir.display().to_string();
    let filter = &app.state.file_picker_filter;

    let items: Vec<ListItem> = app
        .state
        .file_picker_entries
        .iter()
        .enumerate()
        .map(|(i, entry)| {
            let style = if i == app.state.file_picker_selected {
                Style::default().fg(Color::Cyan).bold()
            } else if entry.is_dir {
                Style::default().fg(Color::Blue)
            } else if is_image_file(&entry.path) {
                Style::default().fg(Color::Magenta)
            } else {
                Style::default().dim()
            };
            ListItem::new(Line::from(Span::styled(&entry.name, style)))
        })
        .collect();

    let title = if filter.is_empty() {
        format!(" {dir} ")
    } else {
        format!(" {dir} [filter: {filter}] ")
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(list, area);
}
