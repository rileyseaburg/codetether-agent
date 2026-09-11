//! Verified force termination for an unresponsive mux server.

use anyhow::{Result, bail};
use sysinfo::{Pid, ProcessesToUpdate, Signal, System};

use crate::mux::registry::MuxRecord;

/// Stops the exact process recorded for an unresponsive mux session.
pub(super) async fn run(record: &MuxRecord) -> Result<()> {
    let record = record.clone();
    tokio::task::spawn_blocking(move || terminate(&record))
        .await
        .map_err(|error| anyhow::anyhow!("mux termination task failed: {error}"))?
}

fn terminate(record: &MuxRecord) -> Result<()> {
    let pid = Pid::from_u32(record.pid);
    let mut system = System::new_all();
    let Some(process) = system.process(pid) else {
        return Ok(());
    };
    if !super::terminate_identity::matches(process.cmd(), &record.state.workspace) {
        bail!(
            "refusing to signal PID {}: it is not the mux server for '{}'",
            record.pid,
            record.state.workspace.display()
        );
    }
    let _ = process.kill_with(Signal::Term);
    for _ in 0..20 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
        if system.process(pid).is_none() {
            return Ok(());
        }
    }
    let Some(process) = system.process(pid) else {
        return Ok(());
    };
    if !super::terminate_identity::matches(process.cmd(), &record.state.workspace)
        || !process.kill()
    {
        bail!(
            "failed to terminate mux server for '{}' (PID {})",
            record.state.workspace.display(),
            record.pid
        );
    }
    Ok(())
}
