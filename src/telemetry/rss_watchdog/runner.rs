//! Watchdog sampling loop.

use std::path::PathBuf;
use std::time::Instant;

use super::config::Config;
use super::state::State;
use crate::telemetry::memory::MemorySnapshot;

pub(super) async fn run(spool_dir: PathBuf, config: Config) {
    let mut state = State::default();
    loop {
        tokio::time::sleep(config.sample).await;
        let before = MemorySnapshot::capture();
        let Some(rss_mib) = before.rss_mib() else {
            continue;
        };
        let actions = state.observe(rss_mib, Instant::now(), config);
        if actions.warn {
            tracing::warn!(
                rss_mib,
                threads = before.threads.unwrap_or(0),
                "RSS crossed warning threshold"
            );
        }
        if actions.critical {
            tracing::error!(
                rss_mib,
                threads = before.threads.unwrap_or(0),
                "RSS crossed critical threshold; writing pre-OOM crash report"
            );
            if let Err(error) = super::report::write(&spool_dir, config.critical_mib, &before) {
                tracing::warn!(error = %error, "Failed to write pre-OOM crash report");
            }
        }
        if actions.trim {
            super::reclaim::run(rss_mib);
        }
    }
}
