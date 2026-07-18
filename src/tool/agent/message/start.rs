//! Session setup and task start for one child message.

use super::super::collaboration_runtime::{message_input::MessageImage, thread_status};
use super::super::{bus_publish, helpers};
use crate::session::{Session, SessionEvent};
use anyhow::Result;
use tokio::sync::mpsc;

type Started = (
    mpsc::Receiver<SessionEvent>,
    tokio::task::JoinHandle<Result<Session>>,
);

pub(super) async fn begin(
    agent_id: &str,
    message: String,
    images: Vec<MessageImage>,
    params: &helpers::Params,
) -> Result<Started> {
    super::super::residency::touch(agent_id);
    let mut session =
        thread_status::track_error(agent_id, super::task_session::load(agent_id, params).await)?;
    let registry = thread_status::track_error(agent_id, helpers::get_registry().await)?;
    crate::session::helper::steering::open(agent_id);
    thread_status::running(agent_id);
    bus_publish::announce_working(agent_id, "processing message");
    super::super::event_loop::live_trace::begin(agent_id, message.clone());
    let (tx, rx) = mpsc::channel::<SessionEvent>(256);
    let handle = tokio::spawn(async move {
        session
            .prompt_with_events_and_images(
                &message,
                images.into_iter().map(Into::into).collect(),
                tx,
                registry,
            )
            .await
            .map(|_| session)
    });
    Ok((rx, handle))
}
