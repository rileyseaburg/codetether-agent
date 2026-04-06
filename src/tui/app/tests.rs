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
