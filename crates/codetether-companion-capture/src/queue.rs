use crate::Schedule;
use codetether_companion_protocol::CaptureTrigger;

impl Schedule {
    /// Coalesce local clicks. Ignore disabled/unsupported triggers and all
    /// input while paused. The OS adapter must filter to the selected monitor.
    pub fn queue_click(&mut self, trigger: CaptureTrigger) {
        if let Some(options) = self.options {
            match trigger {
                CaptureTrigger::RightClick if options.right_click => {
                    self.pending_click = Some(trigger);
                }
                CaptureTrigger::DoubleClick if options.double_click => {
                    self.pending_click = Some(trigger);
                }
                _ => {}
            }
        }
    }
    /// Replace/clear the opaque remote request; cannot arm a paused scheduler.
    /// A command with no ID clears stale work. Empty/blank IDs are ignored.
    /// The authenticated relay, not this scheduler, owns request freshness.
    pub fn set_request(&mut self, request_id: Option<String>) {
        if self.options.is_some() {
            self.pending_request = request_id.filter(|id| !id.trim().is_empty());
        }
    }
    /// Borrow the pending ID for a Manual upload only. Never send it with clicks
    /// or periodic frames. Caller must serialize this read with scheduling.
    pub fn request_id(&self) -> Option<&str> {
        self.pending_request.as_deref()
    }
}
