use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::session_runtime;
use crate::tui::app::state::App;

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
