use crate::{Options, Schedule};
use std::time::Duration;

impl Schedule {
    /// Reset and arm only after explicit local consent, live pairing, and monitor
    /// selection. Never call from pairing completion or a remote command.
    /// See the crate example. The host must first cancel previous in-flight work.
    pub fn resume(&mut self, options: Options) {
        *self = Self {
            options: Some(options),
            ..Self::default()
        };
    }
    /// Disarm and discard pending work on pause, lock, disconnect, stop or unpair.
    /// The host must independently cancel capture/upload and revoke credentials.
    pub fn pause(&mut self) {
        *self = Self::default();
    }
    /// Mark a successful upload using the host's monotonic elapsed time.
    /// Clears the single pending click/request and any retry delay.
    pub fn accepted(&mut self, now: Duration) {
        if self.options.is_some() {
            self.last_accepted = Some(now);
            self.pending_click = None;
            self.pending_request = None;
            self.retry_after = None;
        }
    }
    /// Delay a failed upload for five seconds without discarding its request.
    /// A failed upload does not advance the successful-capture clock.
    pub fn failed(&mut self, now: Duration) {
        if self.options.is_some() {
            self.retry_after = Some(now.saturating_add(Duration::from_secs(5)));
        }
    }
}
