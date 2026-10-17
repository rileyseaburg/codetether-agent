//! Initialize a fresh runtime from validated session configuration.
use super::Runtime;
use codetether_companion_protocol::SessionInput;
impl Runtime {
    pub(crate) fn new(input: SessionInput) -> Self {
        Self {
            model: input.model,
            prompt: input.prompt,
            interval: input.interval_seconds,
            text: String::new(),
            previous: String::new(),
            status: "waiting".into(),
            captured_at: None,
            seq: 0,
            frames: 0,
            last_at: 0,
            stopped: false,
            generation: 0,
            active: None,
            viewers: Vec::new(),
            pending: None,
            reply: None,
        }
    }
}
