//! Periodic branch observation inside a remote swarm worker.
//!
//! [start] returns a [Probe] that emits the existing stdout protocol. Call
//! [Probe::finish] before publishing the final task result.
mod metrics;

use std::sync::mpsc;
use std::{thread, time::Duration};

/// Stop sender and join handle for a remote worker probe.
/// # Examples
/// ```
/// let probe = codetether_swarm_k8s::probe::start("example", 0);
/// probe.finish();
/// ```
pub struct Probe {
    stop: mpsc::Sender<()>,
    thread: Option<thread::JoinHandle<()>>,
}

/// Start periodic Git-only branch observations, or disable them with zero.
/// # Arguments
/// Receives the subtask identity and requested interval in seconds.
/// # Returns
/// A probe handle; positive intervals have a three-second minimum.
/// # Panics
/// Panics if the operating system cannot create the observation thread.
pub fn start(subtask_id: &str, interval_secs: u64) -> Probe {
    let (stop, stopped) = mpsc::channel();
    let thread = (interval_secs > 0).then(|| {
        let id = subtask_id.to_string();
        thread::spawn(move || {
            loop {
                emit(&id);
                if stopped
                    .recv_timeout(Duration::from_secs(interval_secs.max(3)))
                    .is_ok()
                {
                    break;
                }
            }
        })
    });
    Probe { stop, thread }
}

impl Probe {
    /// Signal shutdown and wait for the observation thread to finish.
    pub fn finish(self) {
        let _ = self.stop.send(());
        if let Some(thread) = self.thread {
            let _ = thread.join();
        }
    }
}

fn emit(subtask_id: &str) {
    let probe = metrics::snapshot(subtask_id);
    if let Ok(json) = serde_json::to_string(&probe) {
        println!(
            "{}{json}",
            codetether_swarm_core::remote_protocol::SWARM_SUBTASK_PROBE_PREFIX
        );
    }
}
