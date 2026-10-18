//! Application services needed to retain and release prompt-owned leases.
use async_trait::async_trait;
use codetether_worktree_lease::CoordinationReply;
/// Supplies coordinator activity and lease renewal/release operations.
/// # Implementors
/// Applications preserve their authenticated transport and inherited mux identity.
#[async_trait]
pub trait LeaseHost: Send + Sync + 'static {
    /// Whether this prompt inherits an active coordinator.
    fn active(&self) -> bool;
    /// Renew this owner's leases.
    /// # Errors
    /// Propagates transport or coordinator failures for runtime logging.
    async fn renew(&self, owner: &str) -> anyhow::Result<Option<CoordinationReply>>;
    /// Release this owner's leases.
    /// # Errors
    /// Propagates transport or coordinator failures for runtime logging.
    async fn release(&self, owner: &str) -> anyhow::Result<Option<CoordinationReply>>;
}
