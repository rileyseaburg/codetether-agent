//! Local controls stop work before attempting any remote notification.
use super::{
    background,
    state::{OFF, State},
};

impl State {
    pub(super) fn pause(&mut self) {
        background::cancel(self);
        self.background.paused = true;
        self.status = "Sharing OFF — paused locally. Resume requests to re-enable.";
    }
    pub(super) fn resume(&mut self) {
        self.background.paused = false;
        self.status = "Sharing OFF — waiting for pairing, monitor and unlocked desktop.";
    }
    pub(super) fn shutdown(&mut self) {
        background::forget(self);
        self.resources.stop_observers();
        self.selected = None;
        self.monitors.clear();
        self.status = OFF;
    }
}
impl Drop for background::Background {
    fn drop(&mut self) {
        if let Some(worker) = &self.worker {
            worker.stop.cancel();
        }
    }
}
