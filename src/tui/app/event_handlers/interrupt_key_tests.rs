use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::session_runtime;
use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

fn runtime() -> session_runtime::TuiSessionHandle {
    let (event_tx, _) = tokio::sync::mpsc::channel(1);
    session_runtime::spawn(event_tx, tokio::sync::mpsc::channel(1).0)
}

#[tokio::test]
async fn escape_interrupts_processing() {
    let mut app = App::default();
    app.state.processing = true;
    let handled = super::interrupt_key::handle(
        &mut app,
        &runtime(),
        KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
    );
    assert_eq!(handled, Some(false));
    assert!(app.state.status.contains("Cancellation requested"));
}

#[tokio::test]
async fn ctrl_c_interrupts_processing() {
    let mut app = App::default();
    app.state.processing = true;
    let handled = super::interrupt_key::handle(
        &mut app,
        &runtime(),
        KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
    );
    assert_eq!(handled, Some(false));
}

#[tokio::test]
async fn escape_leaves_swarm_monitor_without_interrupting_processing() {
    let mut app = App::default();
    app.state.processing = true;
    app.state.view.mode = ViewMode::Swarm;
    let handled = super::interrupt_key::handle(
        &mut app,
        &runtime(),
        KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
    );
    assert_eq!(handled, None);

    crate::tui::app::navigation::handle_escape(&mut app);
    assert_eq!(app.state.view.mode, ViewMode::Chat);
    assert!(!app.state.status.contains("Cancellation requested"));
}
