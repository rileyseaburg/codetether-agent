//! Session page mutation and persistence contract.
use anyhow::Result;
use async_trait::async_trait;
use codetether_context_policy::pages::PageKind;
use codetether_context_policy::state_header::{ConstraintCorrection, ConstraintRecord};
use codetether_provider_core::Message;
/// Existing session storage with tracked page writes and persistence.
///
/// # Implementors
/// Delegate indexed writes to the session's existing tracked page container.
#[async_trait]
pub trait PinSource: codetether_context_loader::Source {
    /// Number of page entries resident in the ordinary working window.
    fn page_count(&self) -> usize;
    /// Absolute transcript index of the first loaded page.
    fn page_offset(&self) -> usize;
    /// Current classification of a loaded page.
    fn page_kind(&self, index: usize) -> Option<PageKind>;
    /// Owned original source from a resident or prepared slot.
    fn source_message(&self, index: usize) -> Option<Message>;
    /// Resolve an absolute source to an opaque resident or historical slot.
    /// The slot remains valid until preparation or persistence changes the view.
    fn source_index(&self, absolute: usize) -> Option<usize> {
        absolute
            .checked_sub(self.page_offset())
            .filter(|index| *index < self.page_count())
    }
    /// Prepare at most two absolute sources without widening the loaded window.
    /// # Errors
    /// Returns missing source, stale revision or storage errors.
    async fn prepare_sources(&mut self, _indices: &[usize]) -> Result<()> {
        Ok(())
    }
    /// Set one validated slot, preserving resident or historical change tracking.
    fn set_page_kind(&mut self, index: usize, kind: PageKind);
    /// Complete pinned source at an absolute index, including evicted sources.
    fn constraint_source(&self, absolute: usize) -> Option<ConstraintRecord> {
        let index = self.source_index(absolute)?;
        ConstraintRecord::from_message(
            absolute,
            &self.source_message(index)?,
            self.page_kind(index)?,
        )
    }
    /// Persisted exact-span corrections, in insertion order.
    fn corrections(&self) -> &[ConstraintCorrection];
    /// Replace correction metadata as part of the same session save as page edits.
    fn set_corrections(&mut self, corrections: Vec<ConstraintCorrection>);
    /// Persist the modified session.
    /// # Errors
    /// Returns storage write errors.
    async fn persist(&mut self) -> Result<()>;
}
