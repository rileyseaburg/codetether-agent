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

#[tokio::test]
async fn enter_while_local_turn_is_active_steers_it() {
    let _guard = QUEUE_TEST_LOCK.lock().await;
    let _ = prompt_queue::take();
    let mut app = App::default();
    app.state.view_mode = ViewMode::Chat;
    app.state.processing = true;
    app.state.input = "change direction".to_string();
    let cwd = std::path::Path::new(".");
    let session = Session::new().await.expect("session");
    let session_id = session.id.clone();
    let mut slot = SessionSlot::new(session);
    let (event_tx, _) = tokio::sync::mpsc::channel(8);
    let (notice_tx, _) = tokio::sync::mpsc::channel(8);
    let runtime = session_runtime::spawn(event_tx, notice_tx);
    assert!(runtime.activate_steering(&session_id));

    super::handle_enter_chat(&mut app, cwd, &mut slot, &None, &None, &runtime).await;

    assert!(app.state.input.is_empty());
    assert!(prompt_queue::take().is_none());
    let session = slot.borrow_mut().expect("session remains available");
    assert_eq!(crate::session::helper::steering::drain_into(session), 1);
    runtime.clear_steering();
}
