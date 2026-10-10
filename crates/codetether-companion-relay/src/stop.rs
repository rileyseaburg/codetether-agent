//! Terminal stop: abort analysis, emit `stopped`, close streams, clear text.
use crate::{events::event, runtime::Runtime};
use codetether_companion_protocol::EventKind;

impl Runtime {
    /// Abort analysis and discard pending captures/typing when Windows pauses.
    pub(crate) fn pause(&mut self) {
        if let Some((_, cancel)) = &self.active {
            cancel.cancel();
        }
        self.pending = None;
        self.reply = None;
        self.status = "paused".into();
        self.publish_state();
    }
    /// Idempotently stop; analysis and pending questions are discarded.
    pub(crate) fn stop(&mut self) {
        if self.stopped {
            return;
        }
        self.stopped = true;
        self.status = "stopped".into();
        if let Some((_, cancel)) = &self.active {
            cancel.cancel();
        }
        self.publish(event(EventKind::Stopped, None, Some("stopped")));
        self.viewers.clear();
        self.text.clear();
        self.previous.clear();
        self.pending = None;
        self.reply = None;
    }
}
