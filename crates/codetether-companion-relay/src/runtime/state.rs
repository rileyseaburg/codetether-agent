//! Memory-only state shared by the session's behavioral modules.
use super::{Pending, QueuedReply};
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;
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
