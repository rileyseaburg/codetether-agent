//! Manual throughput and retained-RSS benchmark for the replay buffer.

use std::time::Instant;

use super::OutputBuffer;
use crate::telemetry::memory::MemorySnapshot;

const WRITTEN_MIB: usize = 256;

#[test]
#[ignore = "manual performance benchmark"]
fn long_horizon_output_buffer_benchmark() {
    let before = MemorySnapshot::capture().rss_kb.unwrap_or_default();
    let mut buffer = OutputBuffer::new();
    let chunk = vec![b'x'; 8192];
    let started = Instant::now();
    for _ in 0..(WRITTEN_MIB * 1024 * 1024 / chunk.len()) {
        buffer.append(std::hint::black_box(&chunk));
    }
    let elapsed = started.elapsed();
    let after = MemorySnapshot::capture().rss_kb.unwrap_or_default();
    let throughput = WRITTEN_MIB as f64 / elapsed.as_secs_f64();
    println!(
        "BENCH mux_buffer written_mib={WRITTEN_MIB} retained_bytes={} capacity_bytes={} rss_delta_kib={} elapsed_ms={} throughput_mib_s={throughput:.2}",
        buffer.bytes.len(),
        buffer.bytes.capacity(),
        after.saturating_sub(before),
        elapsed.as_millis()
    );
    assert_eq!(buffer.bytes.len(), super::OUTPUT_LIMIT);
}
