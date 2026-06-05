use std::sync::Arc;

use crate::bus::AgentBus;
use crate::session::{Session, TailLoad};

pub(super) enum SessionLoadOutcome {
    Loaded {
        msg_count: usize,
        title: Option<String>,
        dropped: usize,
        file_bytes: u64,
        original_id: Option<String>,
    },
    NewFallback {
        reason: String,
    },
}

pub(super) fn apply_load(
    session: &mut Session,
    bus: &Arc<AgentBus>,
    loaded: anyhow::Result<TailLoad>,
) -> SessionLoadOutcome {
    match loaded {
        Ok(load) => super::session_loaded::outcome(session, bus, load),
        Err(err) => SessionLoadOutcome::NewFallback {
            reason: err.to_string(),
        },
    }
}
