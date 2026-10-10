//! Focus-checked Win32 keyboard delivery; never generates or evaluates text.
mod apartment;
mod editable;
mod focus;
mod foreground;
mod guard;
mod keyboard;
use crate::{Monitor, TypingError, validate_selection};
use std::{
    sync::atomic::AtomicBool,
    thread,
    time::{Duration, Instant},
};

pub(crate) fn type_focused(
    monitor: &Monitor,
    text: &str,
    cancelled: &AtomicBool,
) -> Result<(), TypingError> {
    let deadline = Instant::now() + Duration::from_secs(30);
    guard::active(cancelled, deadline)?;
    validate_selection(monitor).map_err(|_| TypingError::Unavailable)?;
    let _dpi = super::dpi::Guard::enter().map_err(|_| TypingError::Unavailable)?;
    let _apartment = apartment::Apartment::enter()?;
    let target = focus::Target::get(monitor.bounds())?;
    // Send directly to the existing caret; never create a staging window,
    // change focus/selection, or replace the input's contents.
    for character in text.chars() {
        guard::check(monitor, &target, cancelled, deadline)?;
        keyboard::character(character)?;
        thread::sleep(Duration::from_millis(5));
    }
    guard::check(monitor, &target, cancelled, deadline)
}
