//! Map a `ForageUpdate` to a chat message + status line.

use crate::tui::app::state::App;

use super::state::{ForageOffer, ForageUpdate};

/// Convert an update into `(message_text, status_text)`, applying any
/// side effects (clearing `active`, setting a pending offer) on `app`.
pub(super) fn map_update(app: &mut App, update: ForageUpdate) -> (String, String) {
    match update {
        ForageUpdate::Status(s) => (s.clone(), format!("Forage: {s}")),
        ForageUpdate::Complete(s) => {
            app.state.forage.active = false;
            (s, "Forage complete".to_string())
        }
        ForageUpdate::Offer {
            text,
            selected,
            top,
            model,
        } => {
            app.state.forage.active = false;
            app.state.forage.pending_offer = Some(ForageOffer { top, model });
            offer_message(text, selected)
        }
        ForageUpdate::Error(e) => {
            app.state.forage.active = false;
            (format!("Forage error: {e}"), "Forage failed".to_string())
        }
    }
}

fn offer_message(text: String, selected: usize) -> (String, String) {
    let things = if selected == 1 { "thing" } else { "things" };
    (
        format!(
            "{text}\n\nI found {selected} {things} I can work on. \
             Want me to start? Type 'yes' to begin or 'no' to skip."
        ),
        "Start working? (yes / no)".to_string(),
    )
}
