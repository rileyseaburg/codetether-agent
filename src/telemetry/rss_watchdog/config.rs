//! RSS watchdog configuration.

use std::time::Duration;

/// Environment variable for the warning threshold in MiB.
pub const ENV_RSS_WARN_MIB: &str = "CODETETHER_RSS_WARN_MIB";
/// Environment variable for the critical threshold in MiB.
pub const ENV_RSS_CRITICAL_MIB: &str = "CODETETHER_RSS_CRITICAL_MIB";
/// Environment variable for the sampling interval in seconds.
pub const ENV_RSS_SAMPLE_SECS: &str = "CODETETHER_RSS_SAMPLE_SECS";
/// Environment variable for the allocator-trim interval in seconds.
pub const ENV_RSS_TRIM_SECS: &str = "CODETETHER_RSS_TRIM_SECS";

#[derive(Debug, Clone, Copy)]
pub(super) struct Config {
    pub warn_mib: u64,
    pub critical_mib: u64,
    pub sample: Duration,
    pub trim: Duration,
}

impl Config {
    pub fn load() -> Self {
        Self {
            warn_mib: env(ENV_RSS_WARN_MIB, 1024),
            critical_mib: env(ENV_RSS_CRITICAL_MIB, 3072),
            sample: Duration::from_secs(env(ENV_RSS_SAMPLE_SECS, 2).max(1)),
            trim: Duration::from_secs(env(ENV_RSS_TRIM_SECS, 30).max(1)),
        }
    }
}

fn env(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}
