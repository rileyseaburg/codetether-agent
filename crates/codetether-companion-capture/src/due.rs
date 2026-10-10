use crate::Schedule;
use codetether_companion_protocol::CaptureTrigger;
use std::time::Duration;

impl Schedule {
    /// Select work, without dequeuing or starting I/O.
    ///
    /// # Arguments
    /// * `now` - Monotonic elapsed time from the same fixed host epoch.
    /// # Returns
    /// A fresh remote request first, then a queued click (five-second cooldown),
    /// then periodic work. Retry delay gates every trigger. `None` while paused.
    /// Remote requests bypass the successful-upload cooldown, as in the reference.
    /// # Examples
    /// See the crate example. One serialized host loop must perform the capture,
    /// check desktop/session eligibility, upload, then call `accepted` or `failed`.
    /// Never launch concurrent tasks by repeatedly polling this non-consuming API.
    pub fn due(&self, now: Duration) -> Option<CaptureTrigger> {
        let options = self.options?;
        if self.retry_after.is_some_and(|deadline| now < deadline) {
            return None;
        }
        if self.pending_request.is_some() {
            return Some(CaptureTrigger::Manual);
        }
        if self
            .last_accepted
            .is_some_and(|last| now < last || now - last < Duration::from_secs(5))
        {
            return None;
        }
        if let Some(click) = self.pending_click {
            return Some(click);
        }
        if options.periodic
            && self
                .last_accepted
                .is_none_or(|last| now - last >= options.interval)
        {
            return Some(CaptureTrigger::Periodic);
        }
        None
    }
}
