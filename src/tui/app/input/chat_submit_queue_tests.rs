use crate::session::Session;
use crate::tui::app::session_runtime::{self, SessionSlot};
use crate::tui::app::state::{App, prompt_queue};
use crate::tui::models::ViewMode;

static QUEUE_TEST_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

#[tokio::test]
async fn enter_without_local_active_turn_queues_follow_up() {
    let _guard = QUEUE_TEST_LOCK.lock().await;
    let _ = prompt_queue::take();
    let mut app = App::default();
    app.state.view_mode = ViewMode::Chat;
    app.state.processing = true;
    app.state.input = "hello tui".to_string();
    app.state.input_cursor = app.state.input.chars().count();
    let cwd = std::path::Path::new(".");
    let mut slot = SessionSlot::new(Session::new().await.expect("session"));
    let (event_tx, _) = tokio::sync::mpsc::channel(8);
    let (notice_tx, _) = tokio::sync::mpsc::channel(8);
    let runtime = session_runtime::spawn(event_tx, notice_tx);

    super::handle_enter_chat(&mut app, cwd, &mut slot, &None, &None, &runtime).await;

    assert!(app.state.input.is_empty());
    assert_eq!(prompt_queue::take().as_deref(), Some("hello tui"));
}

#[path = "chat_submit_steering_tests.rs"]
mod steering;
