//! Chat-banner notifications for session load outcomes.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::session_outcome::SessionLoadOutcome;

impl SessionLoadOutcome {
    /// Push a visible chat banner for outcomes the user must not miss.
    pub(super) fn push_banner(&self, app: &mut App) {
        match self {
            Self::ScanFailed { reason } => {
                let msg = format!(
                    "⚠ WARNING: Your previous session could not be resumed.\n\
                     Reason: {reason}\n\
                     A new session has been started. Your old session is still on disk."
                );
                app.state
                    .messages
                    .push(ChatMessage::new(MessageType::System, msg));
                app.state.scroll_to_bottom();
            }
            Self::Loaded { dropped, .. } if *dropped > 0 => {
                let msg = format!(
                    "ℹ Session resumed with tail-cap: {dropped} older messages were not \
                     loaded to stay within the context window. Your full history is \
                     preserved on disk."
                );
                app.state
                    .messages
                    .push(ChatMessage::new(MessageType::System, msg));
                app.state.scroll_to_bottom();
            }
            _ => {}
        }
    }
}
