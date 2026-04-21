//! RSS watchdog: samples memory periodically, logs above threshold, and
//! writes a synthetic crash report when RSS crosses a critical level so
//! the existing crash-report flusher ships it through the same pipeline
//! on next startup.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use chrono::Utc;
use serde_json::json;

use super::memory::MemorySnapshot;

/// Environment variables that tune the watchdog.
pub const ENV_RSS_WARN_MIB: &str = "CODETETHER_RSS_WARN_MIB";
pub const ENV_RSS_CRITICAL_MIB: &str = "CODETETHER_RSS_CRITICAL_MIB";
pub const ENV_RSS_SAMPLE_SECS: &str = "CODETETHER_RSS_SAMPLE_SECS";

/// Default warning threshold (MiB) — logs `warn!` once when RSS crosses it.
const DEFAULT_WARN_MIB: u64 = 2048;
/// Default critical threshold (MiB) — writes a breadcrumb once when crossed.
const DEFAULT_CRITICAL_MIB: u64 = 8192;
/// Default sampling interval (seconds).
const DEFAULT_SAMPLE_SECS: u64 = 5;

/// Spawn the watchdog in the background. Idempotent — calling it more
/// than once in the same process is a no-op.
///
/// `spool_dir` is the crash-report spool directory. The synthetic report
/// is written there using the same JSON shape as panic-derived reports so
/// the existing flusher picks it up on next startup.
pub fn spawn(spool_dir: PathBuf) {
    static STARTED: AtomicBool = AtomicBool::new(false);
    if STARTED.swap(true, Ordering::SeqCst) {
        return;
    }
    let warn_mib = read_env_u64(ENV_RSS_WARN_MIB).unwrap_or(DEFAULT_WARN_MIB);
    let critical_mib = read_env_u64(ENV_RSS_CRITICAL_MIB).unwrap_or(DEFAULT_CRITICAL_MIB);
    let sample_secs = read_env_u64(ENV_RSS_SAMPLE_SECS).unwrap_or(DEFAULT_SAMPLE_SECS);

    tokio::spawn(async move {
        run_loop(spool_dir, warn_mib, critical_mib, sample_secs).await;
    });
}

async fn run_loop(spool_dir: PathBuf, warn_mib: u64, critical_mib: u64, sample_secs: u64) {
    let mut warned = false;
    let mut critical_written = false;
    let interval = Duration::from_secs(sample_secs.max(1));
    loop {
        tokio::time::sleep(interval).await;
        let snap = MemorySnapshot::capture();
        let Some(rss_mib) = snap.rss_mib() else {
            continue;
        };

        if !warned && rss_mib >= warn_mib {
            warned = true;
            tracing::warn!(
                rss_mib,
                threads = snap.threads.unwrap_or(0),
                "RSS crossed warning threshold"
            );
        }

        if !critical_written && rss_mib >= critical_mib {
            critical_written = true;
            tracing::error!(
                rss_mib,
                threads = snap.threads.unwrap_or(0),
                "RSS crossed critical threshold; writing pre-OOM crash report"
            );
            let message = format!(
                "pre_oom: RSS {} MiB crossed critical threshold {} MiB (threads={})",
                rss_mib,
                critical_mib,
                snap.threads.unwrap_or(0)
            );
            if let Err(err) = write_pre_oom_report(&spool_dir, &message, &snap) {
                tracing::warn!(error = %err, "Failed to write pre-OOM crash report");
            }
        }
    }
}

/// Write a crash-report-shaped JSON document into `spool_dir` so the
/// existing crash-report flusher ships it on next startup. The shape
/// matches `crash::CrashReport` so no custom server handling is needed.
fn write_pre_oom_report(
    spool_dir: &Path,
    message: &str,
    memory: &MemorySnapshot,
) -> std::io::Result<()> {
    std::fs::create_dir_all(spool_dir)?;
    let now = Utc::now();
    let report_id = uuid::Uuid::new_v4().to_string();
    let thread_name = std::thread::current()
        .name()
        .map(str::to_string)
        .unwrap_or_else(|| "unnamed".to_string());
    let command_line = std::env::args().collect::<Vec<_>>().join(" ");
    let report = json!({
        "report_version": 1,
        "report_id": report_id,
        "occurred_at": now.to_rfc3339(),
        "app_version": env!("CARGO_PKG_VERSION"),
        "command_line": command_line,
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "process_id": std::process::id(),
        "thread_name": thread_name,
        "panic_message": message,
        "panic_location": null,
        "backtrace": "",
        "memory": memory,
    });
    let file_name = format!(
        "{}-{}.json",
        now.format("%Y%m%dT%H%M%S%.3fZ"),
        report_id
    );
    let path = spool_dir.join(file_name);
    let bytes = serde_json::to_vec_pretty(&report)
        .map_err(|e| std::io::Error::other(e.to_string()))?;
    std::fs::write(path, bytes)
}

fn read_env_u64(var: &str) -> Option<u64> {
    std::env::var(var).ok().and_then(|v| v.parse().ok())
}
