//! Active versus completed request-latency badge selection.
use crate::RequestTimings;
use ratatui::text::Span;
/// Build active or completed request-latency badges.
///
/// # Arguments
/// * `timings` - Current and previous request timing values.
/// * `elapsed` - Lazy elapsed-time lookup, invoked only for an active request.
///
/// # Returns
/// Owned timing spans, or none when no request timings exist.
///
/// # Examples
/// ```
/// assert!(codetether_chat_status::latency_badge_spans(Default::default(), || None).is_none());
/// ```
pub fn latency_badge_spans(
    timings: RequestTimings,
    elapsed: impl FnOnce() -> Option<u64>,
) -> Option<Vec<Span<'static>>> {
    if timings.processing {
        crate::timing_active::spans(timings, elapsed)
    } else {
        crate::timing_idle::spans(timings)
    }
}
