//! Loaded-session continuation and fresh-session outcome construction.
use super::Host;
use crate::Resolved;
use anyhow::Result;
use codetether_bus_core::AgentBus;
use codetether_session_resume::{SessionLoadOutcome, is_expected_absence as is_expected};
use codetether_session_store::TailLoad;
use std::sync::Arc;
pub(super) async fn resolve_loaded<H: Host>(
    load: TailLoad<H::Session>,
    bus: &Arc<AgentBus>,
) -> Result<Resolved<H::Session>> {
    let label = H::label(&load.session);
    let dropped = load.dropped;
    let file_bytes = load.file_bytes;
    let mut session = H::with_bus(load.session, bus.clone());
    if dropped > 0 {
        tracing::warn!(target: "codetether_agent::tui::app::run::session_resolve_helpers",
            session_id = %H::id(&session), dropped, file_bytes,
            "session resumed with tail-cap: oldest {dropped} messages not loaded",
        );
    }
    let msg_count = H::history_len(&session);
    let source_id = H::fork_if_truncated(&mut session, dropped).await?;
    Ok(Resolved {
        session,
        outcome: SessionLoadOutcome::Loaded {
            msg_count,
            label,
            dropped,
            source_id,
        },
    })
}

pub(super) async fn resolve_new<H: Host>(
    reason: String,
    bus: &Arc<AgentBus>,
) -> Result<Resolved<H::Session>> {
    let session = H::with_bus(H::new_session().await?, bus.clone());
    let outcome = if is_expected(&reason) {
        tracing::debug!(target: "codetether_agent::tui::app::run::session_resolve_helpers","no prior session for workspace — starting fresh");
        SessionLoadOutcome::Fresh
    } else {
        tracing::warn!(target: "codetether_agent::tui::app::run::session_resolve_helpers", reason = %reason, "session scan failed — starting fresh session");
        SessionLoadOutcome::ScanFailed { reason }
    };
    Ok(Resolved { session, outcome })
}
