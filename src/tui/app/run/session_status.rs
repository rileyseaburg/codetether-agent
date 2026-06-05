use super::session_outcome::SessionLoadOutcome;

impl SessionLoadOutcome {
    pub(super) fn status(&self, session_id: &str) -> String {
        match self {
            Self::Loaded { msg_count: 0, .. } => {
                format!("Loaded session {session_id} (empty - type a message to start)")
            }
            Self::Loaded {
                msg_count,
                title,
                dropped,
                file_bytes,
                original_id,
            } => loaded_status(*msg_count, title, *dropped, *file_bytes, original_id),
            Self::NewFallback { reason } => {
                format!("New session (no prior session for this workspace: {reason})")
            }
        }
    }
}

fn loaded_status(
    msg_count: usize,
    title: &Option<String>,
    dropped: usize,
    file_bytes: u64,
    original_id: &Option<String>,
) -> String {
    let label = title.as_deref().unwrap_or("(untitled)");
    if dropped == 0 {
        return format!("Loaded previous session: {label} - {msg_count} messages");
    }
    let mb = file_bytes as f64 / (1024.0 * 1024.0);
    let orig = original_id.as_deref().unwrap_or("?");
    format!(
        "Large session ({mb:.1} MiB): showing last {msg_count} of {total} entries from \"{label}\". Forked to a new session - original {orig} preserved on disk.",
        total = msg_count + dropped
    )
}
