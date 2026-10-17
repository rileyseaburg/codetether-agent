//! Session page mutation and persistence contract.
use anyhow::Result;
use async_trait::async_trait;
use codetether_context_policy::pages::PageKind;
/// Existing session storage with tracked page writes and persistence.
///
/// # Implementors
/// Delegate indexed writes to the session's existing tracked page container.
#[async_trait]
pub trait PinSource: codetether_context_loader::Source {
    /// Number of page entries that can be addressed.
    fn page_count(&self) -> usize;
    /// Set one validated index, preserving the store's change tracking.
    fn set_page_kind(&mut self, index: usize, kind: PageKind);
    /// Persist the modified session.
    /// # Errors
    /// Returns storage write errors.
    async fn persist(&mut self) -> Result<()>;
}
