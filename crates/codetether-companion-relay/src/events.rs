//! Sequenced SSE publication and terminal stop for a session runtime.
use crate::runtime::Runtime;
use codetether_companion_protocol::{EventKind, ScreenEvent};

/// Build an unsequenced event; `publish` assigns `seq`.
pub(crate) fn event(kind: EventKind, text: Option<String>, status: Option<&str>) -> ScreenEvent {
    ScreenEvent {
        kind,
        seq: 0,
        text,
        status: status.map(str::to_string),
        captured_at: None,
    }
}
pub(crate) fn line(event: &ScreenEvent) -> String {
    format!(
        "data: {}\n\n",
        serde_json::to_string(event).unwrap_or_default()
    )
}
impl Runtime {
    /// Full current state, sent first to every (re)connecting viewer.
    pub(crate) fn snapshot(&self) -> ScreenEvent {
        let mut snapshot = event(
            EventKind::Snapshot,
            Some(self.text.clone()),
            Some(&self.status),
        );
        snapshot.seq = self.seq;
        snapshot.captured_at = self.captured_at.clone();
        snapshot
    }
    /// Sequence and fan out; slow or closed viewers are dropped.
    pub(crate) fn publish(&mut self, mut event: ScreenEvent) {
        self.seq += 1;
        event.seq = self.seq;
        let line = line(&event);
        self.viewers
            .retain(|viewer| viewer.try_send(line.clone()).is_ok());
    }
    /// Publish the current status as a snapshot event.
    pub(crate) fn publish_state(&mut self) {
        let mut state = event(
            EventKind::Snapshot,
            Some(self.text.clone()),
            Some(&self.status),
        );
        state.captured_at = self.captured_at.clone();
        self.publish(state);
    }
}
