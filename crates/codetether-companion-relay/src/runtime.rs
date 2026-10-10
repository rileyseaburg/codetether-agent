//! Per-session analysis state kept beside the core registry.
use codetether_companion_protocol::{DeviceReply, SessionInput};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

/// Owner fresh-frame request; the question never reaches the device.
pub(crate) struct Pending {
    pub id: String,
    pub question: String,
    pub created: i64,
}

/// Owner reply queued for the device to type; re-served until acknowledged.
pub(crate) struct QueuedReply {
    pub id: String,
    pub text: String,
    pub created: i64,
}

/// Memory-only analysis, streaming, and budget state for one session.
pub(crate) struct Runtime {
    pub model: String,
    pub prompt: String,
    pub interval: u16,
    pub text: String,
    pub previous: String,
    pub status: String,
    pub captured_at: Option<String>,
    pub seq: u64,
    pub frames: u32,
    pub last_at: i64,
    pub stopped: bool,
    pub generation: u64,
    pub active: Option<(u64, CancellationToken)>,
    pub viewers: Vec<Sender<String>>,
    pub pending: Option<Pending>,
    pub reply: Option<QueuedReply>,
}
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
