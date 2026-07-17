//! Isolated harness for the actual mux ring modules while Cargo is busy.

use std::time::Instant;
use std::collections::VecDeque;

const OUTPUT_LIMIT: usize = 4 * 1024 * 1024;
const WRITTEN_MIB: usize = 256;

mod mux {
    pub const OUTPUT_LIMIT: usize = super::OUTPUT_LIMIT;

    #[path = "../../src/mux/pty/terminal_mode.rs"]
    mod terminal_mode;
    #[path = "../../src/mux/pty/buffer/replay.rs"]
    mod replay;
    #[path = "../../src/mux/pty/buffer/replay_append.rs"]
    mod replay_append;

    pub fn benchmark_output(written_mib: usize) -> (std::time::Duration, usize) {
        let mut bytes = replay::ReplayBytes::new();
        let mut mode = terminal_mode::TerminalMode::new(false);
        let chunk = vec![b'x'; 8192];
        let started = std::time::Instant::now();
        for _ in 0..(written_mib * 1024 * 1024 / chunk.len()) {
            mode.observe(std::hint::black_box(&chunk));
            bytes.append(&chunk);
        }
        (started.elapsed(), bytes.len())
    }

    pub fn active_after(chunks: &[&[u8]]) -> bool {
        let mut mode = terminal_mode::TerminalMode::new(false);
        for chunk in chunks {
            mode.observe(chunk);
        }
        mode.active()
    }
}

#[path = "../src/mux/pty/buffer/replay.rs"]
mod replay;
#[path = "../src/mux/pty/buffer/replay_append.rs"]
mod replay_append;

#[test]
fn preserves_wrapped_order() {
    let mut bytes = replay::ReplayBytes::new();
    bytes.append(&vec![b'x'; OUTPUT_LIMIT - 2]);
    bytes.append(b"abcdef");
    assert_eq!(bytes.read(OUTPUT_LIMIT - 6, 6), b"abcdef");
}

#[test]
fn detects_split_alternate_screen_transitions() {
    assert!(mux::active_after(&[b"text\x1b[?10", b"49hframe"]));
    assert!(!mux::active_after(&[b"\x1b[?1049h", b"\x1b[?1049l"]));
}

#[test]
#[ignore = "manual performance benchmark"]
fn long_horizon_ring_benchmark() {
    let mut bytes = replay::ReplayBytes::new();
    let chunk = vec![b'x'; 8192];
    let started = Instant::now();
    for _ in 0..(WRITTEN_MIB * 1024 * 1024 / chunk.len()) {
        bytes.append(std::hint::black_box(&chunk));
    }
    let elapsed = started.elapsed();
    let throughput = WRITTEN_MIB as f64 / elapsed.as_secs_f64();
    println!(
        "BENCH mux_ring written_mib={WRITTEN_MIB} retained_bytes={} capacity_bytes={} elapsed_ms={} throughput_mib_s={throughput:.2}",
        bytes.len(),
        bytes.capacity(),
        elapsed.as_millis()
    );
    assert_eq!(bytes.len(), OUTPUT_LIMIT);
}

#[test]
#[ignore = "comparison with the replaced eviction path"]
fn old_vec_deque_benchmark() {
    let mut bytes: VecDeque<u8> = VecDeque::new();
    let chunk = vec![b'x'; 8192];
    let started = Instant::now();
    for _ in 0..(WRITTEN_MIB * 1024 * 1024 / chunk.len()) {
        let excess = (bytes.len() + chunk.len()).saturating_sub(OUTPUT_LIMIT);
        bytes.drain(..excess);
        bytes.extend(&chunk);
    }
    let elapsed = started.elapsed();
    let throughput = WRITTEN_MIB as f64 / elapsed.as_secs_f64();
    println!(
        "BENCH mux_vec_deque elapsed_ms={} throughput_mib_s={throughput:.2}",
        elapsed.as_millis()
    );
    assert_eq!(bytes.len(), OUTPUT_LIMIT);
}

#[test]
#[ignore = "end-to-end output hot-path benchmark"]
fn optimized_output_buffer_benchmark() {
    let (elapsed, retained) = mux::benchmark_output(WRITTEN_MIB);
    let throughput = WRITTEN_MIB as f64 / elapsed.as_secs_f64();
    println!(
        "BENCH mux_output_optimized elapsed_ms={} throughput_mib_s={throughput:.2}",
        elapsed.as_millis()
    );
    assert_eq!(retained, OUTPUT_LIMIT);
}
