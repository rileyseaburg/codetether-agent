//! Tests that UI-only slash commands work while the session is checked out.

use crate::session::Session;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

#[tokio::test]
async fn settings_opens_while_session_checked_out() {
    let mut app = App::default();
    app.state.view_mode = ViewMode::Chat;
    let cwd = std::path::Path::new(".");
    let mut slot = SessionSlot::new(Session::new().await.expect("session"));
    // Simulate an in-flight prompt owning the session.
    let _checked_out = slot.take_for_prompt().expect("session checked out");

    let handled = super::run(&mut app, cwd, &mut slot, &None, "/settings").await;

    assert!(handled, "slash command should be handled");
    assert_eq!(app.state.view_mode, ViewMode::Settings);
}

#[tokio::test]
async fn unknown_command_reports_busy_while_checked_out() {
    let mut app = App::default();
    app.state.view_mode = ViewMode::Chat;
    let cwd = std::path::Path::new(".");
    let mut slot = SessionSlot::new(Session::new().await.expect("session"));
    let _checked_out = slot.take_for_prompt().expect("session checked out");

    let handled = super::run(&mut app, cwd, &mut slot, &None, "/undo").await;

    assert!(handled);
    assert!(app.state.status.contains("busy"));
    assert_eq!(app.state.view_mode, ViewMode::Chat);
}
