# Workspace recall memory benchmark

Measured on Linux 6.8.0 x86_64 with Rust 1.95.0 against the real
`/home/riley/spotlessbinco` recall index: 4,844 sidecars totaling 291 MiB.
Results are single debug-test runs on the same host.

## Command

```bash
CODETETHER_DATA_DIR=/home/riley/spotlessbinco/.codetether-agent \
CODETETHER_BENCH_WORKSPACE=/home/riley/spotlessbinco \
RUSTC_WRAPPER= cargo test --lib workspace_recall_memory_benchmark \
  -- --ignored --nocapture
```

## Results

| Implementation | Hits | Time | Retained RSS delta |
|---|---:|---:|---:|
| Decoded process-global workspace cache | 10 | 9,838 ms | 749,128 KiB |
| Bounded streaming top-K ranker, run 1 | 10 | 9,569 ms | 28,704 KiB |
| Bounded streaming top-K ranker, verification | 10 | 8,917 ms | 27,356 KiB |

The streaming implementation reduced retained memory by 96.2–96.3% and was
2.7–9.4% faster across the two runs. It scanned all cataloged sessions with 16
concurrent sidecar reads; it did not reduce recall coverage or cap the index by
age.

## Live-process evidence

Before the change, active TUI processes retained roughly 680–720 MiB of
private anonymous memory after using recall. Their workspace contained 291 MiB
of JSON sidecars, which expanded into the process-global decoded cache. Agent
steps also produced temporary 1.3–2.5 GiB peaks; the existing RSS watchdog
successfully reclaimed those temporary allocations, but could not reclaim the
live cache.
