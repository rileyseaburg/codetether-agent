//! Pure admission rules for an uploaded frame (no state mutation).
use crate::{ApiError, runtime::Runtime};
use codetether_companion_protocol::{Capture, CaptureTrigger};

/// Reject stopped sessions, mismatched requests, cooldowns, and spent budgets.
pub(crate) fn check(
    rt: &Runtime,
    frame: &Capture,
    captured: i64,
    now: i64,
) -> Result<(), ApiError> {
    if rt.stopped {
        return Err(ApiError::new(410, "Screen session ended"));
    }
    if let Some(p) = &rt.pending
        && (frame.request_id.as_deref() != Some(p.id.as_str()) || captured < p.created)
    {
        return Err(ApiError::new(
            409,
            "A fresh requested screenshot is required",
        ));
    }
    if frame.request_id.is_some() && rt.pending.is_none() {
        return Err(ApiError::new(409, "Capture request is no longer active"));
    }
    let clicked = frame.trigger.is_some_and(|t| t != CaptureTrigger::Periodic);
    let cooldown = if clicked {
        5_000
    } else {
        i64::from(rt.interval) * 1000
    };
    if rt.active.is_some() || (rt.pending.is_none() && now - rt.last_at < cooldown) {
        return Err(ApiError::new(
            409,
            "Analysis busy or capture interval not reached",
        ));
    }
    if rt.frames >= 120 {
        return Err(ApiError::new(
            410,
            "Capture budget reached; create a new session",
        ));
    }
    Ok(())
}
