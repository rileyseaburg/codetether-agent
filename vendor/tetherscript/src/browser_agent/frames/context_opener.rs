use super::WindowOpener;
use crate::browser_agent::context::BrowserContext;
use crate::browser_agent::page::BrowserPage;

impl BrowserContext {
    /// Add a page whose top window has an opener page.
    pub fn open_page_with_opener(
        &mut self,
        opener_index: usize,
        mut page: BrowserPage,
    ) -> Result<usize, String> {
        let opener_page = self
            .page(opener_index)
            .ok_or_else(|| format!("unknown opener page {opener_index}"))?;
        page.set_opener(WindowOpener {
            page_index: opener_index,
            frame_id: opener_page.frame_tree().root_id(),
        });
        Ok(self.new_page(page))
    }
}
