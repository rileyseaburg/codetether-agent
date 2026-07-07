//! Status bar text and chat-banner generation for session load outcomes.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::session_outcome::SessionLoadOutcome;

impl SessionLoadOutcome {
    /// One-line status bar summary shown after startup completes.
    pub(super) fn status(&self, session_id: &str) -> String {
        match self {
            Self::Loaded { msg_count: 0, .. } => {
                format!("Loaded session {session_id} (empty)")
            }
            Self::Loaded {
                msg_count,
                title,
                dropped,
                ..
            } => loaded_status(session_id, *msg_count, title, *dropped),
            Self::Fresh => "New session — type a message to start".to_string(),
            Self::ScanFailed { reason } => {
                format!("⚠ New session (prior session could not be loaded: {reason})")
            }
        }
    }

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

fn loaded_status(
    session_id: &str,
    msg_count: usize,
    title: &Option<String>,
    dropped: usize,
) -> String {
    let label = title.as_deref().unwrap_or(session_id);
    if dropped == 0 {
        format!("Resumed: {label} ({msg_count} messages)")
    } else {
        format!("Resumed: {label} — showing last {msg_count} messages ({dropped} older not loaded)")
    }
}
