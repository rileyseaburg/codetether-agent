use std::sync::Arc;

use crate::bus::AgentBus;
use crate::session::{Session, TailLoad};

use super::session_outcome::SessionLoadOutcome;

pub(super) fn outcome(
    session: &mut Session,
    bus: &Arc<AgentBus>,
    load: TailLoad,
) -> SessionLoadOutcome {
    let title = load.session.title.clone();
    let dropped = load.dropped;
    let file_bytes = load.file_bytes;
    *session = load.session.with_bus(bus.clone());
    let original_id = crate::tui::app::session_fork::fork_if_truncated(session, dropped);
    log_fork(session, dropped, file_bytes, original_id.as_deref());
    SessionLoadOutcome::Loaded {
        msg_count: session.history().len(),
        title,
        dropped,
        file_bytes,
        original_id,
    }
}

fn log_fork(session: &Session, dropped: usize, file_bytes: u64, original_id: Option<&str>) {
    if let Some(original) = original_id {
        tracing::warn!(
            original_id = %original,
            new_id = %session.id,
            dropped,
            file_bytes,
            "forked large session on resume to protect on-disk history"
        );
    }
}
