use super::commands::handle_slash_command;
use super::model_picker::{apply_selected_model, close_model_picker};
use super::state::{App, AppState};
use crate::session::Session;
use crate::tui::chat::message::MessageType;
use crate::tui::models::ViewMode;

#[tokio::test]
async fn slash_model_opens_model_picker_without_registry() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");

    handle_slash_command(
        &mut app,
        std::path::Path::new("."),
        &mut session,
        None,
        "/model",
    )
    .await;

    assert_eq!(app.state.view_mode, ViewMode::Model);
    assert!(app.state.model_picker_active);
    assert!(app.state.available_models.is_empty());
    assert_eq!(app.state.status, "No models available");
}

#[test]
fn model_picker_filters_and_selects() {
    let mut state = AppState {
        available_models: vec![
            "zai/glm-5".to_string(),
            "openai/gpt-4o".to_string(),
            "github-copilot/gpt-4o".to_string(),
        ],
        ..Default::default()
    };

    state.open_model_picker();
    state.model_filter_push('g');
    state.model_filter_push('l');

    let filtered = state.filtered_models();
    assert_eq!(filtered, vec!["zai/glm-5"]);
    assert_eq!(state.selected_model(), Some("zai/glm-5"));
}

#[tokio::test]
async fn apply_selected_model_persists_to_session() {
    let mut app = App::default();
    app.state.available_models = vec!["zai/glm-5".to_string(), "openai/gpt-4o".to_string()];
    app.state.open_model_picker();
    app.state.selected_model_index = 1;
    app.state.set_view_mode(ViewMode::Model);

    let mut session = Session::new().await.expect("session should create");
    apply_selected_model(&mut app, &mut session);

    assert_eq!(session.metadata.model.as_deref(), Some("openai/gpt-4o"));
    assert_eq!(app.state.view_mode, ViewMode::Chat);
    assert!(!app.state.model_picker_active);
}

#[test]
fn closing_model_picker_returns_to_chat() {
    let mut app = App::default();
    app.state.open_model_picker();
    app.state.set_view_mode(ViewMode::Model);

    close_model_picker(&mut app);

    assert_eq!(app.state.view_mode, ViewMode::Chat);
    assert!(!app.state.model_picker_active);
}

#[tokio::test]
async fn slash_model_alias_opens_model_picker() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");

    handle_slash_command(
        &mut app,
        std::path::Path::new("."),
        &mut session,
        None,
        "/m",
    )
    .await;

    assert_eq!(app.state.view_mode, ViewMode::Model);
    assert!(app.state.model_picker_active);
}

#[tokio::test]
async fn slash_latency_opens_latency_view() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");

    handle_slash_command(
        &mut app,
        std::path::Path::new("."),
        &mut session,
        None,
        "/latency",
    )
    .await;

    assert_eq!(app.state.view_mode, ViewMode::Latency);
    assert_eq!(app.state.status, "Latency inspector");
}

#[tokio::test]
async fn slash_file_path_attaches_file_to_composer() {
    let temp = tempfile::tempdir().expect("tempdir should create");
    let file_path = temp.path().join("notes.md");
    std::fs::write(&file_path, "# Notes\nship it").expect("fixture should write");

    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");

    handle_slash_command(
        &mut app,
        temp.path(),
        &mut session,
        None,
        &format!("/file {}", file_path.display()),
    )
    .await;

    assert!(app.state.input.contains("Shared file: notes.md"));
    assert!(app.state.input.contains("# Notes"));
    assert!(matches!(
        app.state.messages.last().map(|msg| &msg.message_type),
        Some(MessageType::System)
    ));
}

#[tokio::test]
async fn slash_image_supported_extension_attaches() {
    let temp = tempfile::tempdir().expect("tempdir should create");
    let img_path = temp.path().join("photo.png");
    // Write a minimal valid 1x1 PNG
    let png_bytes: &[u8] = &[
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE, // 8-bit RGB
        0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, // IDAT chunk
        0x08, 0xD7, 0x63, 0xF8, 0xCF, 0xC0, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xE2, 0x21,
        0xBC, 0x33, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60,
        0x82, // IEND
    ];
    std::fs::write(&img_path, png_bytes).expect("fixture should write");

    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");

    handle_slash_command(
        &mut app,
        temp.path(),
        &mut session,
        None,
        &format!("/image {}", img_path.display()),
    )
    .await;

    assert_eq!(app.state.pending_images.len(), 1);
    assert!(app.state.status.contains("Attached"));
    assert!(matches!(
        app.state.messages.last().map(|msg| &msg.message_type),
        Some(MessageType::System)
    ));
}

#[tokio::test]
async fn slash_image_empty_arg_shows_usage() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");

    handle_slash_command(
        &mut app,
        std::path::Path::new("."),
        &mut session,
        None,
        "/image",
    )
    .await;

    assert!(app.state.status.contains("Usage: /image"));
    assert!(app.state.pending_images.is_empty());
}

#[tokio::test]
async fn slash_image_missing_file_shows_error() {
    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");

    handle_slash_command(
        &mut app,
        std::path::Path::new("."),
        &mut session,
        None,
        "/image /nonexistent/photo.png",
    )
    .await;

    assert!(app.state.pending_images.is_empty());
    assert!(matches!(
        app.state.messages.last().map(|msg| &msg.message_type),
        Some(MessageType::System)
    ));
    let last_msg = app.state.messages.last().unwrap();
    assert!(last_msg.content.contains("Failed to attach image"));
}

#[tokio::test]
async fn slash_image_unsupported_extension_shows_error() {
    let temp = tempfile::tempdir().expect("tempdir should create");
    let txt_path = temp.path().join("readme.txt");
    std::fs::write(&txt_path, "not an image").expect("fixture should write");

    let mut app = App::default();
    let mut session = Session::new().await.expect("session should create");

    handle_slash_command(
        &mut app,
        temp.path(),
        &mut session,
        None,
        &format!("/image {}", txt_path.display()),
    )
    .await;

    assert!(app.state.pending_images.is_empty());
    let last_msg = app.state.messages.last().unwrap();
    assert!(last_msg.content.contains("Unsupported image format"));
}
