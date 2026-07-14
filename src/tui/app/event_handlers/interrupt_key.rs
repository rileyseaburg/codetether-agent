//! Priority handling for keys that interrupt an active turn.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::tui::app::session_runtime::TuiSessionHandle;
use crate::tui::app::state::App;

pub(super) fn handle(app: &mut App, runtime: &TuiSessionHandle, key: KeyEvent) -> Option<bool> {
    if !app.state.processing {
        return None;
    }
    let ctrl_c =
        matches!(key.code, KeyCode::Char('c')) && key.modifiers.contains(KeyModifiers::CONTROL);
    if !ctrl_c && !matches!(key.code, KeyCode::Esc) {
        return None;
    }
    runtime.request_cancel_current();
    app.state.status = "Cancellation requested — partial turn will be saved.".to_string();
    Some(false)
}
