//! MultiAgentV2 wait timeout policy.

use anyhow::{Result, bail};
use std::time::Duration;

const MIN_MS: u64 = 10_000;
const MAX_MS: u64 = 3_600_000;

pub(super) fn duration(timeout_ms: u64) -> Result<Duration> {
    if timeout_ms < MIN_MS {
        bail!("timeout_ms must be at least {MIN_MS}");
    }
    if timeout_ms > MAX_MS {
        bail!("timeout_ms must be at most {MAX_MS}");
    }
    Ok(Duration::from_millis(timeout_ms))
}

#[cfg(test)]
#[path = "timeout_tests.rs"]
mod tests;
