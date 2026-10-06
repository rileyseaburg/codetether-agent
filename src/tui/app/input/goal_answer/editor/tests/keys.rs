//! Native event-dispatch helper; no provider request or file save occurs.

use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
pub(super) use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::path::Path;

pub(super) async fn press(
    app: &mut App,
    slot: &mut SessionSlot,
    runtime: &TuiSessionHandle,
    code: KeyCode,
    modifiers: KeyModifiers,
) {
    crate::tui::app::event_handlers::handle_event(
        app,
        Path::new("."),
        slot,
        &None,
        &None,
        runtime,
        KeyEvent::new(code, modifiers),
    )
    .await
    .unwrap();
}

pub(super) async fn end(app: &mut App, slot: &mut SessionSlot, runtime: &TuiSessionHandle) {
    press(app, slot, runtime, KeyCode::End, KeyModifiers::NONE).await;
}

pub(super) async fn save(app: &mut App, slot: &mut SessionSlot, runtime: &TuiSessionHandle) {
    press(
        app,
        slot,
        runtime,
        KeyCode::Char('s'),
        KeyModifiers::CONTROL,
    )
    .await;
}

pub(super) async fn escape(app: &mut App, slot: &mut SessionSlot, runtime: &TuiSessionHandle) {
    press(app, slot, runtime, KeyCode::Esc, KeyModifiers::NONE).await;
}
