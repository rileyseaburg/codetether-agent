//! Standard-thread launcher for background RLM jobs.

use tokio::runtime::Handle;

use super::{cache, gate, job, status};

pub(super) fn spawn(job: job::Job) {
    let key = job.key;
    let notify = job.notify.clone();
    let Some(permit) = gate::try_acquire() else {
        cache::fail(key);
        status::progress(&notify, "background busy; using bounded output", 0, 1);
        return;
    };
    let Ok(handle) = Handle::try_current() else {
        cache::fail(key);
        drop(permit);
        tracing::warn!("RLM background summary skipped: no async runtime handle");
        return;
    };
    let spawn = std::thread::Builder::new()
        .name("codetether-rlm-bg".into())
        .spawn(move || {
            let _permit = permit;
            handle.block_on(job::run(job));
        });
    if let Err(e) = spawn {
        cache::fail(key);
        status::complete(&notify, false, Some(e.to_string()));
        tracing::warn!(error = %e, "RLM background summary thread spawn failed");
    }
}
