//! Optional lifecycle for Kubernetes branch-observation probes.

use std::sync::mpsc;
use std::{thread, time::Duration};

pub(super) struct Probe {
    stop: mpsc::Sender<()>,
    thread: Option<thread::JoinHandle<()>>,
}

pub(super) fn start(subtask_id: &str, interval_secs: u64) -> Probe {
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
    pub(super) fn finish(self) {
        let _ = self.stop.send(());
        if let Some(thread) = self.thread {
            let _ = thread.join();
        }
    }
}

fn emit(subtask_id: &str) {
    let probe = super::remote_probe_metrics::snapshot(subtask_id);
    if let Ok(json) = serde_json::to_string(&probe) {
        println!(
            "{}{json}",
            super::kubernetes_executor::SWARM_SUBTASK_PROBE_PREFIX
        );
    }
}
