//! Status bar text generation for session load outcomes.

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
        format!(
            "Resumed: {label} — showing last {msg_count} messages; scroll up for {dropped} older"
        )
    }
}
