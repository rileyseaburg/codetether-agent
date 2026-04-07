use std::path::Path;
use std::sync::Arc;

use base64::Engine;
use crossterm::event::KeyModifiers;
use std::io::Cursor;
use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session, SessionEvent};
use crate::tui::app::commands::handle_slash_command;
use crate::tui::app::message_text::sync_messages_from_session;
use crate::tui::app::session_sync::{refresh_sessions, return_to_chat};
use crate::tui::app::settings::toggle_selected_setting;
use crate::tui::app::state::App;
use crate::tui::app::symbols::{refresh_symbol_search, symbol_search_active};
use crate::tui::app::worker_bridge::{handle_processing_started, handle_processing_stopped};
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::worker_bridge::TuiWorkerBridge;

/// Check if we're likely running in an SSH/headless session where system
/// clipboard is unavailable.
fn is_ssh_or_headless() -> bool {
    std::env::var("SSH_CONNECTION").is_ok()
        || std::env::var("SSH_TTY").is_ok()
        || (std::env::var("TERM").ok().map_or(false, |t| t.starts_with("xterm"))
            && std::env::var("DISPLAY").is_err()
            && std::env::var("WAYLAND_DISPLAY").is_err())
}

/// Try to get an image from the system clipboard. Returns `Ok(attachment)` on
/// success, or `Err(message)` with a human-readable explanation (e.g. SSH
/// clipboard unavailability) on failure.
pub(crate) fn get_clipboard_image() -> Result<ImageAttachment, String> {
    if is_ssh_or_headless() {
        return Err(
            "Clipboard image unavailable in SSH/headless sessions. \
             Use /image <path> to attach an image file instead."
                .to_string(),
        );
    }

    let mut clipboard = arboard::Clipboard::new().map_err(|e| {
        tracing::debug!(error = %e, "System clipboard unavailable");
        format!(
            "Clipboard unavailable: {e}. Use /image <path> to attach an image file."
        )
    })?;

    let image_data = clipboard.get_image().map_err(|e| {
        tracing::debug!(error = %e, "No image in clipboard");
        "No image found in clipboard. Use /image <path> to attach an image file." .to_string()
    })?;

    let width = image_data.width;
    let height = image_data.height;
    let raw_bytes = image_data.bytes.into_owned();

    let image_buffer: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::from_raw(width as u32, height as u32, raw_bytes)
            .ok_or_else(|| "Failed to create image buffer from clipboard data" .to_string())?;

    let mut png_bytes = Vec::new();
    let mut cursor = Cursor::new(&mut png_bytes);
    image_buffer
        .write_to(&mut cursor, image::ImageFormat::Png)
        .map_err(|e| format!("Failed to encode image as PNG: {e}"))?;

    let base64_data = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    Ok(ImageAttachment {
        data_url: format!("data:image/png;base64,{base64_data}"),
        mime_type: Some("image/png".to_string()),
    })
}

/// Read an image file from disk, encode it as base64, and return it as an
/// `ImageAttachment` ready to send with a message.
pub(crate) fn attach_image_file(path: &Path) -> Result<ImageAttachment, String> {
    if !path.exists() {
        return Err(format!("File not found: {}", path.display()));
    }

    if !path.is_file() {
        return Err(format!("Not a file: {}", path.display()));
    }

    let bytes = std::fs::read(path)
        .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;

    let mime_type = guess_image_mime(path)
        .ok_or_else(|| {
            format!(
                "Unsupported image format: {}. Supported: png, jpg/jpeg, gif, webp, bmp",
                path.display()
            )
        })?;

    let base64_data = base64::engine::general_purpose::STANDARD.encode(&bytes);
    let size_kb = bytes.len() as f64 / 1024.0;
    tracing::info!(
        path = %path.display(),
        mime = %mime_type,
        size_kb = %.1,
        "Attached image file"
    );

    Ok(ImageAttachment {
        data_url: format!("data:{mime_type};base64,{base64_data}"),
        mime_type: Some(mime_type),
    })
}

/// Guess the MIME type for common image file extensions.
fn guess_image_mime(path: &Path) -> Option<String> {
    match path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .as_deref()
    {
        Some("png") => Some("image/png".to_string()),
        Some("jpg") | Some("jpeg") => Some("image/jpeg".to_string()),
        Some("gif") => Some("image/gif".to_string()),
        Some("webp") => Some("image/webp".to_string()),
        Some("bmp") => Some("image/bmp".to_string()),
        Some("svg") => Some("image/svg+xml".to_string()),
        _ => None,
    }
}

pub async fn handle_enter(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    match app.state.view_mode {
        ViewMode::Sessions => handle_enter_sessions(app, cwd, session).await,
        ViewMode::FilePicker => crate::tui::app::file_picker::file_picker_enter(app, cwd),
        ViewMode::Swarm => app.state.swarm.enter_detail(),
        ViewMode::Ralph => app.state.ralph.enter_detail(),
        ViewMode::Bus if app.state.bus_log.filter_input_mode => handle_enter_bus_filter(app),
        ViewMode::Bus => app.state.bus_log.enter_detail(),
        ViewMode::Chat => {
            handle_enter_chat(
                app,
                cwd,
                session,
                registry,
                worker_bridge,
                event_tx,
                result_tx,
            )
            .await
        }
        ViewMode::Model => crate::tui::app::model_picker::apply_selected_model(app, session),
        ViewMode::Settings => toggle_selected_setting(app, session).await,
        ViewMode::Lsp | ViewMode::Rlm | ViewMode::Latency | ViewMode::Protocol => {}
    }
}

pub async fn handle_backspace(app: &mut App) {
    if symbol_search_active(app) {
        app.state.symbol_search.handle_backspace();
        refresh_symbol_search(app).await;
    } else if app.state.view_mode == ViewMode::Bus && app.state.bus_log.filter_input_mode {
        app.state.bus_log.pop_filter_char();
        app.state.status = if app.state.bus_log.filter.is_empty() {
            "Protocol filter cleared".to_string()
        } else {
            format!("Protocol filter: {}", app.state.bus_log.filter)
        };
    } else if app.state.view_mode == ViewMode::Model {
        app.state.model_filter_backspace();
    } else if app.state.view_mode == ViewMode::FilePicker {
        crate::tui::app::file_picker::file_picker_filter_backspace(app);
    } else if app.state.view_mode == ViewMode::Chat {
        app.state.delete_backspace();
        if app.state.input.is_empty() {
            app.state.input_mode = InputMode::Normal;
        } else if app.state.input.starts_with('/') {
            app.state.input_mode = InputMode::Command;
        }
    }
}

pub fn handle_bus_g(app: &mut App) {
    let len = app.state.bus_log.visible_count();
    if len > 0 {
        app.state.bus_log.selected_index = len - 1;
        app.state.bus_log.auto_scroll = true;
    }
}

pub fn handle_bus_c(app: &mut App) {
    app.state.bus_log.clear_filter();
    app.state.status = "Protocol filter cleared".to_string();
}

pub fn handle_bus_slash(app: &mut App) {
    app.state.bus_log.enter_filter_mode();
    app.state.status = "Protocol filter mode".to_string();
}

pub fn handle_sessions_char(app: &mut App, modifiers: KeyModifiers, c: char) {
    if !modifiers.contains(KeyModifiers::CONTROL) && !modifiers.contains(KeyModifiers::ALT) {
        app.state.session_filter_push(c);
    }
}

pub async fn handle_char(app: &mut App, modifiers: KeyModifiers, c: char) {
    if !modifiers.contains(KeyModifiers::CONTROL)
        && !modifiers.contains(KeyModifiers::ALT)
        && symbol_search_active(app)
    {
        app.state.symbol_search.handle_char(c);
        refresh_symbol_search(app).await;
    } else if app.state.view_mode == ViewMode::Bus
        && app.state.bus_log.filter_input_mode
        && !modifiers.contains(KeyModifiers::CONTROL)
        && !modifiers.contains(KeyModifiers::ALT)
    {
        app.state.bus_log.push_filter_char(c);
        app.state.status = format!("Protocol filter: {}", app.state.bus_log.filter);
    } else if app.state.view_mode == ViewMode::Model
        && !modifiers.contains(KeyModifiers::CONTROL)
        && !modifiers.contains(KeyModifiers::ALT)
    {
        app.state.model_filter_push(c);
    } else if app.state.view_mode == ViewMode::FilePicker
        && !modifiers.contains(KeyModifiers::CONTROL)
        && !modifiers.contains(KeyModifiers::ALT)
    {
        crate::tui::app::file_picker::file_picker_filter_push(app, c);
    } else if app.state.view_mode == ViewMode::Chat
        && !modifiers.contains(KeyModifiers::CONTROL)
        && !modifiers.contains(KeyModifiers::ALT)
    {
        app.state.input_mode = if app.state.input.is_empty() && c == '/' {
            InputMode::Command
        } else if app.state.input.starts_with('/') || c == '/' {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        app.state.insert_char(c);
    }
}

pub async fn handle_paste(app: &mut App, text: &str) {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");

    if symbol_search_active(app) {
        for ch in normalized.chars().filter(|ch| *ch != '\n') {
            app.state.symbol_search.handle_char(ch);
        }
        refresh_symbol_search(app).await;
        return;
    }

    if app.state.view_mode == ViewMode::Bus && app.state.bus_log.filter_input_mode {
        for ch in normalized.chars().filter(|ch| *ch != '\n') {
            app.state.bus_log.push_filter_char(ch);
        }
        app.state.status = format!("Protocol filter: {}", app.state.bus_log.filter);
        return;
    }

    if app.state.view_mode == ViewMode::Model {
        for ch in normalized.chars().filter(|ch| *ch != '\n') {
            app.state.model_filter_push(ch);
        }
        return;
    }

    if app.state.view_mode == ViewMode::Sessions {
        for ch in normalized.chars().filter(|ch| *ch != '\n') {
            app.state.session_filter_push(ch);
        }
        return;
    }

    if app.state.view_mode == ViewMode::Chat {
        app.state.input_mode = if app.state.input.is_empty() && normalized.starts_with('/') {
            InputMode::Command
        } else if app.state.input.starts_with('/') {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        app.state.insert_text(&normalized);

        let line_count = normalized.lines().count();
        app.state.status = if line_count > 1 {
            format!("Pasted {line_count} lines into input")
        } else {
            "Pasted into input".to_string()
        };
    }
}

async fn handle_enter_sessions(app: &mut App, cwd: &Path, session: &mut Session) {
    let session_id = app
        .state
        .filtered_sessions()
        .get(app.state.selected_session)
        .map(|(orig_idx, _)| app.state.sessions[*orig_idx].id.clone());
    if let Some(session_id) = session_id {
        crate::tui::app::codex_sessions::load_selected_session(app, cwd, session, &session_id)
            .await;
    }
}

fn handle_enter_bus_filter(app: &mut App) {
    app.state.bus_log.exit_filter_mode();
    app.state.status = if app.state.bus_log.filter.is_empty() {
        "Protocol filter cleared".to_string()
    } else {
        format!("Protocol filter applied: {}", app.state.bus_log.filter)
    };
}

async fn handle_enter_chat(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Option<Arc<ProviderRegistry>>,
    worker_bridge: &Option<TuiWorkerBridge>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    if app.state.processing {
        app.state.status = "Still processing previous request…".to_string();
        return;
    }

    let prompt = app.state.input.trim().to_string();
    if !prompt.is_empty() {
        app.state.push_history(prompt.clone());
    }

    if prompt.starts_with('/') {
        handle_slash_command(app, cwd, session, registry.as_ref(), &prompt).await;
        app.state.clear_input();
        return;
    }

    let pending_images = std::mem::take(&mut app.state.pending_images);

    if prompt.is_empty() && pending_images.is_empty() {
        return;
    }

    app.state
        .messages
        .push(ChatMessage::new(MessageType::User, prompt.clone()));
    for image in &pending_images {
        app.state.messages.push(ChatMessage::new(
            MessageType::Image {
                url: image.data_url.clone(),
            },
            image.data_url.clone(),
        ));
    }
    app.state.clear_input();
    handle_processing_started(app, worker_bridge).await;
    app.state.begin_request_timing();
    // Store the prompt for the watchdog timer to auto-restart if needed.
    app.state.main_inflight_prompt = Some(prompt.clone());

    app.state.status = "Submitting prompt…".to_string();
    app.state.scroll_to_bottom();

    if let Some(registry) = registry {
        session.metadata.auto_apply_edits = app.state.auto_apply_edits;
        let mut session_for_task = session.clone();
        let event_tx = event_tx.clone();
        let result_tx = result_tx.clone();
        let registry = Arc::clone(registry);
        tokio::spawn(async move {
            let result = session_for_task
                .prompt_with_events_and_images(&prompt, pending_images, event_tx, registry)
                .await
                .map(|_| session_for_task);
            let _ = result_tx.send(result).await;
        });
    } else {
        handle_processing_stopped(app, worker_bridge).await;
        app.state.clear_request_timing();
        app.state.messages.push(ChatMessage::new(
            MessageType::Error,
            "No providers available. Configure credentials first (for example: `codetether auth codex` or `codetether auth copilot`).",
        ));
        app.state.status = "No providers configured".to_string();
        app.state.scroll_to_bottom();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tui::chat::message::MessageType;
    use crate::tui::models::ViewMode;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn paste_keeps_multiline_text_in_single_chat_input() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;

        handle_paste(&mut app, "first line\nsecond line").await;

        assert_eq!(app.state.input, "first line\nsecond line");
        assert_eq!(app.state.status, "Pasted 2 lines into input");
    }

    #[tokio::test]
    async fn enter_echoes_user_message_before_provider_failure() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.input = "hello tui".to_string();
        app.state.input_cursor = app.state.input.chars().count();

        let cwd = std::path::Path::new(".");
        let mut session = Session::new().await.expect("session should create");
        let (event_tx, _event_rx) = mpsc::channel(8);
        let (result_tx, _result_rx) = mpsc::channel(8);

        handle_enter(
            &mut app,
            cwd,
            &mut session,
            &None,
            &None,
            &event_tx,
            &result_tx,
        )
        .await;

        assert!(matches!(
            app.state.messages.first().map(|msg| &msg.message_type),
            Some(MessageType::User)
        ));
        assert_eq!(app.state.messages[0].content, "hello tui");
        assert!(matches!(
            app.state.messages.get(1).map(|msg| &msg.message_type),
            Some(MessageType::Error)
        ));
        assert!(app.state.input.is_empty());
        assert!(app.state.processing_started_at.is_none());
    }

    #[tokio::test]
    async fn enter_with_pending_image_sends_even_without_text() {
        let mut app = App::default();
        app.state.view_mode = ViewMode::Chat;
        app.state.pending_images.push(ImageAttachment {
            data_url: "data:image/png;base64,Zm9v".to_string(),
            mime_type: Some("image/png".to_string()),
        });

        let cwd = std::path::Path::new(".");
        let mut session = Session::new().await.expect("session should create");
        let (event_tx, _event_rx) = mpsc::channel(8);
        let (result_tx, _result_rx) = mpsc::channel(8);

        handle_enter(
            &mut app,
            cwd,
            &mut session,
            &None,
            &None,
            &event_tx,
            &result_tx,
        )
        .await;

        assert!(matches!(
            app.state.messages.first().map(|msg| &msg.message_type),
            Some(MessageType::User)
        ));
        assert_eq!(app.state.messages[0].content, "");
        assert!(matches!(
            app.state.messages.get(1).map(|msg| &msg.message_type),
            Some(MessageType::Image { .. })
        ));
        assert!(app.state.pending_images.is_empty());
    }
}
