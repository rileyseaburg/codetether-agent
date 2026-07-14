# Runtime Memory Controls

CodeTether bounds allocator fragmentation and runtime concurrency before RSS
approaches the host OOM limit. These controls complement the single-allocation
guard; they do not replace application-level bounds on live data.

## Prevention

On Linux with glibc, the global allocator wrapper applies two policies before
the first ordinary allocation:

- At most four glibc arenas, preventing short-lived worker threads from
  retaining one large heap each.
- A 128 KiB trim threshold, encouraging glibc to return unused top-of-heap
  pages promptly.

The main Tokio executor uses eight workers. MinIO's async-std dependency uses a
dedicated two-thread executor instead of creating one worker per CPU.

## Reclamation And Evidence

The RSS watchdog samples every two seconds. Once RSS reaches the warning
threshold, it asks glibc to release wholly unused pages immediately and every
30 seconds while RSS remains elevated. Each attempt logs RSS before and after
reclamation.

At the critical threshold, the watchdog writes the pre-reclamation snapshot to
the crash-report spool before trimming. This ordering preserves evidence even
when reclamation succeeds.

## Configuration

| Variable | Default | Range | Purpose |
|---|---:|---:|---|
| `CODETETHER_MALLOC_ARENA_MAX` | 4 | 1–32 | Maximum glibc arenas |
| `CODETETHER_MALLOC_TRIM_KIB` | 128 | 16–1,048,576 | Automatic trim threshold |
| `CODETETHER_RSS_WARN_MIB` | 1024 | positive integer | Start reclamation and warn |
| `CODETETHER_RSS_CRITICAL_MIB` | 3072 | positive integer | Persist pre-OOM evidence |
| `CODETETHER_RSS_SAMPLE_SECS` | 2 | at least 1 | RSS sampling interval |
| `CODETETHER_RSS_TRIM_SECS` | 30 | at least 1 | Reclamation interval |
| `CODETETHER_S3_RUNTIME_THREADS` | 2 | 1–8 | MinIO executor size |

Allocator tuning and explicit trimming are no-ops on non-glibc platforms. The
watchdog and concurrency bounds remain active.
