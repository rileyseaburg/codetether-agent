# Mux long-horizon benchmarks

Measured on Linux 6.8.0 x86_64, AMD Threadripper 3960X, with Rust 1.95.0.
The baseline is commit `8597ab69`; results are single debug-test runs and RSS is
reported as an observed delta, not a stable allocator guarantee.

## Commands

```bash
RUSTC_WRAPPER= cargo test --lib long_horizon_output_buffer_benchmark -- --ignored --nocapture
RUSTC_WRAPPER= cargo test --lib idle_read_requests_one_second -- --ignored --nocapture
RUSTC_WRAPPER= cargo test --lib live_program_scaling_benchmark -- --ignored --nocapture
RUSTC_WRAPPER= cargo test --lib mux
```

## Results

| Scenario | Baseline | Optimized | Change |
|---|---:|---:|---:|
| Idle reads in about 1 second | 57 | 1 | -98.2% |
| Replay-buffer RSS delta | 10,664 KiB | 6,668 KiB | -37.5% |
| Replay-buffer throughput | 15,369 MiB/s | 20,769 MiB/s | +35.1% |
| Threads added by 24 live programs | 48 | 25 | -47.9% |

Both replay-buffer runs retained 4,194,304 bytes after ingesting 256 MiB. The
optimized buffer capacity was exactly 4,194,304 bytes. In the live-program test,
RSS moved from a 6,496 KiB delta to 7,412 KiB; that noisy result is not claimed
as an improvement. The measurable gain there is the deterministic thread count.

## Correctness evidence

The focused suite completed with 30 passed, zero failed, and four intentionally
ignored manual benchmarks. It covers detach/reconnect replay, concurrent control
input during a pending output read, multi-process mux sessions, buffer bounds,
protocol compatibility, and collection of exited child processes.
