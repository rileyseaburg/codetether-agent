use crate::Options;
use codetether_companion_protocol::CaptureTrigger;
use std::time::Duration;

/// Bounded, single-consumer scheduler; stores at most one click and request ID.
/// No credentials, images, or owner questions belong here. Deliberately omits
/// `Debug` so a pending request ID cannot accidentally reach diagnostic logs.
///
/// ```
/// use codetether_companion_capture::Schedule;
/// use std::time::Duration;
/// let schedule = Schedule::default();
/// assert_eq!(schedule.due(Duration::ZERO), None);
/// ```
#[derive(Default)]
pub struct Schedule {
    pub(crate) options: Option<Options>,
    pub(crate) last_accepted: Option<Duration>,
    pub(crate) retry_after: Option<Duration>,
    pub(crate) pending_click: Option<CaptureTrigger>,
    pub(crate) pending_request: Option<String>,
}
