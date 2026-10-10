//! Serial keyboard delivery, separate from capture and from text generation.
use super::work_stop::Stop;
use crate::relay::{Device, Error, Reply};
use codetether_companion_desktop::{Monitor, TypingError, type_in_focused_input};
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

pub(super) async fn deliver(
    device: &Device,
    monitor: &Monitor,
    stop: &Stop,
    phase: &Arc<AtomicU8>,
    reply: Reply,
) -> Result<(), Error> {
    stop.check()?;
    let id = reply.id;
    // Poll failures must not burn an unattempted reply. Reserve only before input.
    let current = device.poll(&stop.token).await?;
    if !current
        .reply
        .as_ref()
        .is_some_and(|r| r.id == id && r.text == reply.text)
    {
        return Ok(());
    }
    drop(current);
    stop.check()?;
    if device.reserve_reply(&id)? {
        phase.store(4, Ordering::Release);
        let selected = monitor.clone();
        let flag = stop.flag.clone();
        let text = reply.text;
        let result =
            tokio::task::spawn_blocking(move || type_in_focused_input(&selected, &text, &flag))
                .await;
        phase.store(
            match result {
                Ok(Ok(())) => 5,
                Ok(Err(TypingError::InvalidText)) => 7,
                Ok(Err(_)) | Err(_) => 6,
            },
            Ordering::Release,
        );
    }
    stop.check()?;
    // The server's ack clears either a typed OR refused attempt; never retry input.
    device.ack_reply(&id, &stop.token).await
}
