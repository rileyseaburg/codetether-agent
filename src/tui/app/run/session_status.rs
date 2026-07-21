//! Status bar text generation for session load outcomes.

use super::session_outcome::SessionLoadOutcome;

impl SessionLoadOutcome {
    /// One-line status bar summary shown after startup completes.
    pub(super) fn status(&self) -> String {
        match self {
            Self::Loaded {
                msg_count: 0,
                label,
                ..
            } => format!("Loaded session {label} (empty)"),
            Self::Loaded {
                msg_count,
                label,
                dropped,
                ..
            } => loaded_status(label, *msg_count, *dropped),
            Self::Fresh => "New session — type a message to start".to_string(),
            Self::ScanFailed { reason } => {
                format!("⚠ New session (prior session could not be loaded: {reason})")
            }
        }
    }
}

fn loaded_status(label: &str, msg_count: usize, dropped: usize) -> String {
    if dropped == 0 {
        format!("Resumed: {label} ({msg_count} messages)")
    } else {
        format!(
            "Resumed: {label} — showing last {msg_count} messages; scroll up for {dropped} older"
        )
    }
}
