use super::{focus::Target, keyboard};
use crate::{Monitor, TypingError, validate_selection};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

pub(super) fn check(
    monitor: &Monitor,
    target: &Target,
    cancelled: &AtomicBool,
    deadline: Instant,
) -> Result<(), TypingError> {
    active(cancelled, deadline)?;
    validate_selection(monitor).map_err(|_| TypingError::Unavailable)?;
    keyboard::quiet()?;
    target.check(monitor.bounds())?;
    keyboard::quiet()?;
    active(cancelled, deadline)
}
pub(super) fn active(cancelled: &AtomicBool, deadline: Instant) -> Result<(), TypingError> {
    if cancelled.load(Ordering::Acquire) || Instant::now() >= deadline {
        Err(TypingError::Cancelled)
    } else {
        Ok(())
    }
}
