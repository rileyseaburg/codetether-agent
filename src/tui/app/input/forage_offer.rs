//! Handle a user's Y/N answer to a pending forage "want me to start?" offer.

use crate::session::Session;
use crate::tui::app::session_runtime::SessionSlot;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::forage_run::{build_tui_forage_args, state::ForageUpdate};

/// Intercept a chat prompt when a forage offer is pending.
///
/// Borrows the active session from `slot` and delegates to
/// [`handle_offer_answer`]. Returns `true` when the prompt was consumed.
pub(super) fn intercept(app: &mut App, slot: &SessionSlot, prompt: &str) -> bool {
    let Some(session) = slot.borrow() else {
        return false;
    };
    handle_offer_answer(app, session, prompt)
}

/// If a forage offer is pending, consume `prompt` as its answer.
///
/// Returns `true` when the prompt was handled (an offer was pending), so the
/// caller should stop further input processing. Affirmative answers relaunch
/// forage in execute mode; anything else dismisses the offer.
pub(super) fn handle_offer_answer(app: &mut App, session: &Session, prompt: &str) -> bool {
    let Some(offer) = app.state.forage.pending_offer.take() else {
        return false;
    };
    if is_affirmative(prompt) {
        start_execution(app, session, offer.top, offer.model);
    } else {
        push_system(app, "No problem — I'll leave it for now.");
        app.state.status = "Forage offer dismissed".to_string();
    }
    true
}

fn start_execution(app: &mut App, session: &Session, top: usize, model: Option<String>) {
    let (tx, rx) = tokio::sync::mpsc::channel::<ForageUpdate>(64);
    app.state.forage.attach_rx(rx);
    push_system(app, "Great — starting work now. I'll keep you posted.");
    app.state.status = "Forage running: executing top selections".to_string();
    let model = model.or_else(|| session.metadata.model.clone());
    crate::tui::forage_run::spawn::spawn_forage_run(build_tui_forage_args(top, true, model), tx);
}

fn push_system(app: &mut App, text: &str) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, text.to_string()));
    app.state.scroll_to_bottom();
}

/// Liberal yes-detector so non-technical users aren't tripped by phrasing.
fn is_affirmative(s: &str) -> bool {
    let t = s.trim().to_lowercase();
    matches!(
        t.as_str(),
        "y" | "yes" | "yeah" | "yep" | "sure" | "ok" | "okay" | "go" | "do it" | "start" | "please"
    )
}

#[cfg(test)]
#[path = "forage_offer_tests.rs"]
mod tests;
