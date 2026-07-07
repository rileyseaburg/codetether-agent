//! Helpers for `session_resolve`: loaded-session and fresh-session builders.

use std::sync::Arc;

use anyhow::Result;

use crate::bus::AgentBus;
use crate::session::{Session, TailLoad};

use super::session_outcome::SessionLoadOutcome;
use super::session_resolve::Resolved;

pub(super) fn resolve_loaded(load: TailLoad, bus: &Arc<AgentBus>) -> Resolved {
    let title = load.session.title.clone();
    let dropped = load.dropped;
    let file_bytes = load.file_bytes;
    let session = load.session.with_bus(bus.clone());
    if dropped > 0 {
        tracing::warn!(
            session_id = %session.id, dropped, file_bytes,
            "session resumed with tail-cap: oldest {dropped} messages not loaded",
        );
    }
    let msg_count = session.history().len();
    Resolved {
        session,
        outcome: SessionLoadOutcome::Loaded {
            msg_count,
            title,
            dropped,
        },
    }
}

pub(super) async fn resolve_new(reason: String, bus: &Arc<AgentBus>) -> Result<Resolved> {
    let session = Session::new().await?.with_bus(bus.clone());
    let outcome = if is_expected(&reason) {
        tracing::debug!("no prior session for workspace — starting fresh");
        SessionLoadOutcome::Fresh
    } else {
        tracing::warn!(reason = %reason, "session scan failed — starting fresh session");
        SessionLoadOutcome::ScanFailed { reason }
    };
    Ok(Resolved { session, outcome })
}

fn is_expected(reason: &str) -> bool {
    reason.contains("requested fresh session")
        || reason.contains("no session")
        || reason.contains("No such file")
        || reason.contains("not found")
}
