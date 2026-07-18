//! Child-session preparation and initial prompt task creation.

use super::super::collaboration_runtime::thread_status;
use super::super::{bus_publish, helpers, store};
use crate::session::{Session, SessionEvent};
use anyhow::{Context, Result};
use tokio::sync::mpsc;

type Started = (
    mpsc::Receiver<SessionEvent>,
    tokio::task::JoinHandle<Result<Session>>,
);

pub(super) async fn begin(agent_id: &str) -> Result<Started> {
    let entry = thread_status::track_error(
        agent_id,
        store::get(agent_id).context(format!("Agent {agent_id} vanished after spawn")),
    )?;
    let kickoff = format!("Assigned task:\n\n{}", entry.instructions);
    let registry = thread_status::track_error(agent_id, helpers::get_registry().await)?;
    let mut session = entry.session;
    if session.bus.is_none() {
        session.bus = crate::bus::global();
    }
    crate::session::helper::steering::open(agent_id);
    thread_status::running(agent_id);
    bus_publish::announce_working(agent_id, "starting first turn");
    super::super::event_loop::live_trace::begin(agent_id, kickoff.clone());
    let (tx, rx) = mpsc::channel::<SessionEvent>(256);
    let handle = tokio::spawn(async move {
        session
            .prompt_with_events(&kickoff, tx, registry)
            .await
            .map(|_| session)
    });
    Ok((rx, handle))
}
