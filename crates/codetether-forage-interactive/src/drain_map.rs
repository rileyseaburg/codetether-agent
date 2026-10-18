//! Map a `ForageUpdate` to a chat message + status line.

use super::ForageState;

use super::state::ForageUpdate;

/// Convert an update into `(message_text, status_text)`, applying any
/// side effects (such as clearing `active`) on `state`.
pub(super) fn map_update(state: &mut ForageState, update: ForageUpdate) -> (String, String) {
    match update {
        ForageUpdate::Status(s) => (s.clone(), format!("Forage: {s}")),
        ForageUpdate::ScanComplete(s) => {
            state.active = false;
            (scan_message(s), "Forage scan complete".to_string())
        }
        ForageUpdate::Complete(s) => {
            state.active = false;
            (s, "Forage complete".to_string())
        }
        ForageUpdate::Error(e) => {
            state.active = false;
            (format!("Forage error: {e}"), "Forage failed".to_string())
        }
    }
}

fn scan_message(text: String) -> String {
    format!(
        "{text}\n\nScan complete; no work was executed. \
         Run `/forage execute` to scan and execute the top selections."
    )
}

#[cfg(test)]
#[path = "drain_map_tests.rs"]
mod tests;
