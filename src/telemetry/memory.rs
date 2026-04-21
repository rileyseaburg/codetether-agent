//! Process memory telemetry.
//!
//! Reads `/proc/self/status` on Linux to surface VmRSS / VmPeak / Threads.
//! All fields are best-effort; a missing or unreadable field is reported as
//! `None` rather than failing the caller.

use serde::{Deserialize, Serialize};

/// Snapshot of a process's memory usage, in kilobytes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MemorySnapshot {
    /// Resident set size, in KiB.
    pub rss_kb: Option<u64>,
    /// Peak resident set size, in KiB.
    pub peak_rss_kb: Option<u64>,
    /// Virtual memory size, in KiB.
    pub vsize_kb: Option<u64>,
    /// Peak virtual memory size, in KiB.
    pub peak_vsize_kb: Option<u64>,
    /// Current thread count for the process.
    pub threads: Option<u64>,
}

impl MemorySnapshot {
    /// Capture a snapshot of the current process's memory usage.
    ///
    /// Returns a default-filled snapshot (all `None`) on non-Linux targets
    /// or when `/proc/self/status` is unreadable.
    pub fn capture() -> Self {
        #[cfg(target_os = "linux")]
        {
            match std::fs::read_to_string("/proc/self/status") {
                Ok(text) => Self::parse(&text),
                Err(_) => Self::default(),
            }
        }
        #[cfg(not(target_os = "linux"))]
        {
            Self::default()
        }
    }

    /// Parse `/proc/self/status`-style content.
    ///
    /// Only `VmRSS`, `VmHWM`, `VmSize`, `VmPeak`, and `Threads` are extracted.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::telemetry::memory::MemorySnapshot;
    ///
    /// let text = "VmRSS:\t 1234 kB\nVmHWM:\t 2345 kB\nThreads:\t   7\n";
    /// let snap = MemorySnapshot::parse(text);
    /// assert_eq!(snap.rss_kb, Some(1234));
    /// assert_eq!(snap.peak_rss_kb, Some(2345));
    /// assert_eq!(snap.threads, Some(7));
    /// ```
    pub fn parse(text: &str) -> Self {
        let mut s = Self::default();
        for line in text.lines() {
            let (key, rest) = match line.split_once(':') {
                Some((k, v)) => (k.trim(), v.trim()),
                None => continue,
            };
            // Values like "1234 kB" or bare "7".
            let num = rest
                .split_whitespace()
                .next()
                .and_then(|n| n.parse::<u64>().ok());
            match key {
                "VmRSS" => s.rss_kb = num,
                "VmHWM" => s.peak_rss_kb = num,
                "VmSize" => s.vsize_kb = num,
                "VmPeak" => s.peak_vsize_kb = num,
                "Threads" => s.threads = num,
                _ => {}
            }
        }
        s
    }

    /// RSS in MiB, rounded down. Returns `None` if RSS is unavailable.
    pub fn rss_mib(&self) -> Option<u64> {
        self.rss_kb.map(|kb| kb / 1024)
    }
}
