use super::{BrowserFrame, FrameTree};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Return iframe metadata scanned from the current document.
    ///
    /// This is a data-model view only. Child frames do not yet own independent
    /// JavaScript runtimes or navigation state.
    pub fn frame_tree(&self) -> FrameTree {
        FrameTree::from_document(self.session.url.clone(), &self.session.document)
    }

    /// Return all frames in deterministic tree order.
    pub fn frames(&self) -> Vec<BrowserFrame> {
        self.frame_tree().frames().to_vec()
    }
}
