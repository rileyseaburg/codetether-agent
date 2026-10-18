//! Native mailbox effects coupled to manifest recovery and owner migration.
use crate::RuntimeHost;
use anyhow::Result;
use async_trait::async_trait;
/// Existing durable-mailbox operations used by recovery transactions.
/// # Implementors
/// Implement [RuntimeHost] and delegate these operations to the mailbox owner.
#[async_trait]
pub trait Host: RuntimeHost {
    /// Restore queued items; report whether any remain, propagating I/O errors.
    async fn hydrate_mailbox(id: &str) -> Result<bool>;
    /// Dispatch the next queued item after successful hydration.
    fn dispatch_next(id: String);
    /// Transfer mailbox ownership, propagating persistence errors.
    async fn reparent_mailbox(id: &str, from: &str, to: &str) -> Result<()>;
}
