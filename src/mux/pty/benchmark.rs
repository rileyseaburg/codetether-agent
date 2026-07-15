//! Manual Linux benchmark for mux program thread and resident-memory scaling.

use std::time::Duration;

use super::{PtyRegistry, TerminalSize};
use crate::telemetry::memory::MemorySnapshot;

const PROGRAMS: u64 = 24;

#[test]
#[ignore = "manual performance benchmark"]
fn live_program_scaling_benchmark() {
    let workspace = std::env::current_dir().unwrap();
    let registry = PtyRegistry::new();
    let before_threads = thread_count();
    let before_rss = MemorySnapshot::capture().rss_kb.unwrap_or_default();
    for id in 0..PROGRAMS {
        registry
            .start(id, "sleep 20", &workspace, TerminalSize::new(80, 24))
            .unwrap();
    }
    std::thread::sleep(Duration::from_millis(100));
    let after_threads = thread_count();
    let after_rss = MemorySnapshot::capture().rss_kb.unwrap_or_default();
    println!(
        "BENCH mux_programs programs={PROGRAMS} thread_delta={} rss_delta_kib={}",
        after_threads.saturating_sub(before_threads),
        after_rss.saturating_sub(before_rss)
    );
    registry.stop_all();
    assert!(after_threads >= before_threads + PROGRAMS as usize);
}

fn thread_count() -> usize {
    std::fs::read_dir("/proc/self/task").unwrap().count()
}
