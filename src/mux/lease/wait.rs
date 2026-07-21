//! Bounded, notification-driven waiting for an overlapping lease to clear.

use super::{CoordinationReply, LeaseRegistry};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

impl LeaseRegistry {
    pub(in crate::mux) async fn acquire_wait(
        &self,
        owner: &str,
        agent: &str,
        workspace: &Path,
        paths: Vec<PathBuf>,
        wait_ms: u64,
    ) -> CoordinationReply {
        let started = Instant::now();
        let limit = Duration::from_millis(wait_ms.min(super::ACQUIRE_WAIT_MILLIS));
        let mut changes = self.subscribe();
        loop {
            let reply = self.acquire(owner, agent, workspace, paths.clone());
            let retry_ms = match &reply {
                CoordinationReply::Blocked { retry_after_ms, .. } => *retry_after_ms,
                _ => return reply.with_waited(elapsed(started)),
            };
            let remaining = limit.saturating_sub(started.elapsed());
            if remaining.is_zero() {
                return reply.with_waited(elapsed(started));
            }
            let expiry = Duration::from_millis(retry_ms.max(1));
            tokio::select! {
                _ = changes.changed() => {}
                _ = tokio::time::sleep(expiry.min(remaining)) => {}
            }
        }
    }
}

fn elapsed(started: Instant) -> u64 {
    started.elapsed().as_millis() as u64
}
