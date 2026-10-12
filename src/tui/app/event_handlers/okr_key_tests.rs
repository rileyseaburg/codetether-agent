use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{handle, is_action};
use crate::tui::app::okr_gate::PendingOkrApproval;
use crate::tui::app::state::App;

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn recognizes_advertised_okr_decision_keys() {
    assert!(is_action(key(KeyCode::Char('a'))));
    assert!(is_action(key(KeyCode::Char('A'))));
    assert!(is_action(key(KeyCode::Char('d'))));
    assert!(is_action(key(KeyCode::Char('D'))));
}

#[test]
fn leaves_unrelated_and_modified_keys_alone() {
    assert!(!is_action(key(KeyCode::Char('x'))));
    assert!(!is_action(key(KeyCode::Enter)));
    assert!(!is_action(KeyEvent::new(
        KeyCode::Char('a'),
        KeyModifiers::CONTROL,
    )));
}

#[tokio::test]
async fn deny_key_consumes_pending_okr() {
    let mut app = App::default();
    app.state.pending_okr_approval = Some(PendingOkrApproval::new(
        "audit the TUI".into(),
        3,
        "test/model".into(),
    ));

    assert!(handle(&mut app, key(KeyCode::Char('d'))).await);
    assert!(app.state.pending_okr_approval.is_none());
    assert_eq!(app.state.status, "OKR denied. Relay cancelled.");
}
