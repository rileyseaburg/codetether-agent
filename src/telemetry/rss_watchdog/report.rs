//! Pre-OOM crash-report persistence.

use std::path::Path;

use chrono::Utc;
use serde_json::json;

use crate::telemetry::memory::MemorySnapshot;

pub(super) fn write(
    spool_dir: &Path,
    critical_mib: u64,
    memory: &MemorySnapshot,
) -> std::io::Result<()> {
    std::fs::create_dir_all(spool_dir)?;
    let now = Utc::now();
    let report_id = uuid::Uuid::new_v4().to_string();
    let rss_mib = memory.rss_mib().unwrap_or(0);
    let threads = memory.threads.unwrap_or(0);
    let report = json!({
        "report_version": 1,
        "report_id": report_id,
        "occurred_at": now.to_rfc3339(),
        "app_version": env!("CARGO_PKG_VERSION"),
        "command_line": std::env::args().collect::<Vec<_>>().join(" "),
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "process_id": std::process::id(),
        "thread_name": std::thread::current().name().unwrap_or("unnamed"),
        "panic_message": format!(
            "pre_oom: RSS {rss_mib} MiB crossed critical threshold {critical_mib} MiB (threads={threads})"
        ),
        "panic_location": null,
        "backtrace": "",
        "memory": memory,
    });
    let name = format!("{}-{report_id}.json", now.format("%Y%m%dT%H%M%S%.3fZ"));
    let bytes = serde_json::to_vec_pretty(&report).map_err(std::io::Error::other)?;
    std::fs::write(spool_dir.join(name), bytes)
}
