//! High-RSS reclamation measurement and logging.

use crate::telemetry::memory::MemorySnapshot;

pub(super) fn run(before_mib: u64) {
    let released = crate::telemetry::system_allocator::trim();
    let after_mib = MemorySnapshot::capture().rss_mib().unwrap_or(before_mib);
    tracing::info!(
        before_mib,
        after_mib,
        reclaimed_mib = before_mib.saturating_sub(after_mib),
        released,
        "Completed high-RSS allocator reclamation"
    );
}
