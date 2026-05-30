//! Trip handler for the [capacity-guarding allocator](super::alloc_guard).
//!
//! Runs only when a single allocation exceeds the ceiling — never on a
//! healthy process. It captures a backtrace and spools a crash report in the
//! same JSON shape the panic flusher already drains, then aborts. Because we
//! are about to abort anyway, the small allocations made here (backtrace,
//! JSON) are fine: they are far below the ceiling and pass straight through.

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::Utc;
use serde_json::json;

use super::alloc_guard::SPOOL_DIR;
use super::memory::MemorySnapshot;

/// Handle an over-ceiling allocation: diagnose, spool, and abort. Marked
/// cold and never-inline so it stays entirely off the allocator hot path.
#[cold]
#[inline(never)]
pub(crate) fn trip(size: usize, ceiling: usize) -> ! {
    // Guard against re-entrancy: if spooling itself tripped the guard, just
    // abort rather than recursing.
    static IN_TRIP: AtomicBool = AtomicBool::new(false);
    if IN_TRIP.swap(true, Ordering::SeqCst) {
        std::process::abort();
    }

    let backtrace = std::backtrace::Backtrace::force_capture().to_string();
    eprintln!(
        "\nFATAL: single allocation of {size} bytes exceeds the {ceiling}-byte alloc \
         guard ceiling.\nThis is a runaway capacity computation, not real data — \
         aborting cleanly with a crash report instead of risking an OS OOM kill.\n\n{backtrace}"
    );
    if let Some(dir) = SPOOL_DIR.get() {
        let _ = write_report(dir, size, ceiling, &backtrace);
    }
    std::process::abort();
}

fn write_report(dir: &Path, size: usize, ceiling: usize, backtrace: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    let report_id = uuid::Uuid::new_v4().to_string();
    let report = json!({
        "report_version": 1,
        "report_id": report_id,
        "occurred_at": Utc::now().to_rfc3339(),
        "app_version": env!("CARGO_PKG_VERSION"),
        "command_line": std::env::args().collect::<Vec<_>>().join(" "),
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "process_id": std::process::id(),
        "thread_name": std::thread::current().name().unwrap_or("unnamed"),
        "panic_message": format!(
            "alloc_guard: single allocation of {size} bytes exceeds ceiling {ceiling} bytes"
        ),
        "panic_location": null,
        "backtrace": backtrace,
        "memory": MemorySnapshot::capture(),
    });
    let bytes = serde_json::to_vec_pretty(&report).map_err(std::io::Error::other)?;
    std::fs::write(dir.join(format!("alloc-guard-{report_id}.json")), bytes)
}
