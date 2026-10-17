//! Captures only synchronous emissions belonging to the current test.
use super::EventSink;
use std::sync::{Arc, Mutex};

type CapturedEvent = (String, serde_json::Value);
type CapturedEvents = Arc<Mutex<Vec<CapturedEvent>>>;

/// Builds a capture callback that ignores unrelated parallel test emitters.
pub(super) fn capture() -> (CapturedEvents, EventSink) {
    let owner = std::thread::current().id();
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink_seen = Arc::clone(&seen);
    let sink: EventSink = Arc::new(move |text, event| {
        // Other tests may emit through helpers without installing a sink.
        if std::thread::current().id() == owner
            && let Some(event) = event
        {
            sink_seen.lock().unwrap().push((text, event));
        }
    });
    (seen, sink)
}
