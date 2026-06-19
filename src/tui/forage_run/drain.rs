//! Drain background forage updates into the chat view.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::drain_map::map_update;
use super::state::ForageUpdate;

/// Drain pending forage updates into chat messages on the background tick.
///
/// Returns `true` when at least one update was applied so the caller can
/// gate a redraw.
pub fn drain_forage_updates(app: &mut App) -> bool {
    let updates: Vec<ForageUpdate> = app
        .state
        .forage
        .rx
        .as_mut()
        .map(|rx| {
            let mut out = Vec::new();
            while let Ok(u) = rx.try_recv() {
                out.push(u);
            }
            out
        })
        .unwrap_or_default();
    if updates.is_empty() {
        return false;
    }
    for update in updates {
        apply_forage_update(app, update);
    }
    true
}

fn apply_forage_update(app: &mut App, update: ForageUpdate) {
    let (text, status) = map_update(app, update);
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, text));
    app.state.status = status;
    app.state.scroll_to_bottom();
}
