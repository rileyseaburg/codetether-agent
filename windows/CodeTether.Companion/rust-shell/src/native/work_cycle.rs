//! One request, one fresh frame, one upload; no screenshot backlog.
use super::work_stop::Stop;
use crate::relay::{Device, Error};
use codetether_companion_desktop::{Monitor, capture_selected, validate_selection};
use codetether_companion_protocol::CaptureTrigger;
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

pub(super) enum Issue {
    Relay(Error),
    Desktop,
}
impl From<Error> for Issue {
    fn from(error: Error) -> Self {
        Self::Relay(error)
    }
}
pub(super) async fn cycle(
    device: &Device,
    monitor: &Monitor,
    stop: &Stop,
    phase: &Arc<AtomicU8>,
    last: Option<&str>,
) -> Result<Option<String>, Issue> {
    stop.check()?;
    let command = device.poll(&stop.token).await?;
    if let Some(reply) = command.reply {
        super::work_reply::deliver(device, monitor, stop, phase, reply).await?;
        return Ok(None);
    }
    let Some(id) = command.request_id else {
        return Ok(None);
    };
    if last == Some(id.as_str()) {
        return Ok(None);
    }
    stop.check()?;
    phase.store(1, Ordering::Release);
    let selected = monitor.clone();
    let flag = stop.flag.clone();
    let frame = tokio::task::spawn_blocking(move || {
        let captured = chrono::Utc::now();
        capture_selected(&selected, &flag).map(|frame| (frame, captured))
    })
    .await
    .map_err(|_| Issue::Desktop)?
    .map_err(|_| Issue::Desktop)?;
    stop.check()?;
    validate_selection(monitor).map_err(|_| Issue::Desktop)?;
    if device.poll(&stop.token).await?.request_id.as_deref() != Some(id.as_str()) {
        return Ok(None);
    }
    validate_selection(monitor).map_err(|_| Issue::Desktop)?;
    stop.check()?;
    phase.store(2, Ordering::Release);
    device
        .upload(
            &frame.0,
            frame.1,
            CaptureTrigger::Manual,
            Some(id.clone()),
            &stop.token,
        )
        .await?;
    Ok(Some(id))
}
