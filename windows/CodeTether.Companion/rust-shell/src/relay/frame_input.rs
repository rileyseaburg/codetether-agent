//! Bound frame metadata before encoding or issuing network work.
use super::{Error, validation};
use chrono::{DateTime, Utc};
use codetether_companion_desktop::CapturedFrame;
use codetether_companion_protocol::CaptureTrigger;

pub(super) fn validate(
    frame: &CapturedFrame,
    captured: DateTime<Utc>,
    trigger: CaptureTrigger,
    id: Option<&str>,
) -> Result<(), Error> {
    let age = Utc::now()
        .signed_duration_since(captured)
        .num_milliseconds();
    if !(-60_000..=300_000).contains(&age)
        || !(24..=524_288).contains(&frame.jpeg().len())
        || frame.width() == 0
        || frame.height() == 0
        || frame.width() > 1920
        || frame.height() > 1920
        || !frame.jpeg().starts_with(&[0xff, 0xd8])
        || !frame.jpeg().ends_with(&[0xff, 0xd9])
        || matches!(trigger, CaptureTrigger::Manual) != id.is_some()
    {
        return Err(Error::InvalidInput);
    }
    if let Some(id) = id {
        validation::request_id(id).map_err(|_| Error::InvalidInput)?;
    }
    Ok(())
}
