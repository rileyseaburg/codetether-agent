//! Preserve the existing tracked page write and save operations.
use crate::session::{Session, pages::PageKind};
#[async_trait::async_trait]
impl codetether_context_pin::PinSource for Session {
    fn page_count(&self) -> usize {
        self.pages.len()
    }
    fn set_page_kind(&mut self, index: usize, kind: PageKind) {
        self.pages[index] = kind;
    }
    async fn persist(&mut self) -> anyhow::Result<()> {
        Session::save(self).await
    }
}
