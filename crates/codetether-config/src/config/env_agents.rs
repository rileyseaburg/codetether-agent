//! Legacy environment overlays for sub-agent resource limits.

use crate::config::Config;
use std::num::NonZeroUsize;

impl Config {
    pub(super) fn apply_agent_env(&mut self) {
        if let Some(limit) = parse_env::<NonZeroUsize>("CODETETHER_AGENT_MAX_THREADS") {
            self.agents.max_concurrent_threads_per_session = Some(limit);
        }
        if let Some(limit) = parse_env::<u8>("CODETETHER_AGENT_MAX_DEPTH") {
            self.agents.max_depth = Some(limit);
        }
    }
}

fn parse_env<T>(name: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    let value = std::env::var(name).ok()?;
    match value.parse() {
        Ok(parsed) => Some(parsed),
        Err(_) => {
            tracing::warn!(variable = name, value = %value, "Invalid agent limit override");
            None
        }
    }
}
