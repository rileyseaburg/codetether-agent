//! Bounded, notification-driven waiting for an overlapping lease to clear.

use super::{CoordinationReply, LeaseRegistry};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

impl LeaseRegistry {
    /// Acquire paths, waiting for release or expiry within the capped budget.
    /// # Arguments
    /// `owner`, `agent`, `workspace` and `paths` describe the claim; `wait_ms`
    /// is capped at [crate::ACQUIRE_WAIT_MILLIS].
    /// # Returns
    /// The final acquisition reply annotated with elapsed wait time.
    /// # Panics
    /// Panics if the lease-table mutex is poisoned.
    /// # Examples
    /// ```no_run
    /// # async fn example() {
    /// let registry = codetether_worktree_lease::LeaseRegistry::new();
    /// let _reply = registry.acquire_wait("turn", "agent", std::path::Path::new("/repo"), vec!["a.rs".into()], 100).await;
    /// # }
    /// ```
    pub async fn acquire_wait(
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
