use super::{FrameId, FrameWindowRelation, WindowOpener};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Return opener metadata for this page, if it was opened by another page.
    pub fn opener(&self) -> Option<WindowOpener> {
        self.frame_windows.opener()
    }

    /// Set opener metadata for this page.
    pub fn set_opener(&mut self, opener: WindowOpener) {
        self.frame_windows.set_opener(opener);
    }

    /// Clear opener metadata for this page.
    pub fn clear_opener(&mut self) {
        self.frame_windows.clear_opener();
    }

    /// Return parent, top, and opener metadata for `frame_id`.
    pub fn frame_window_relation(&self, frame_id: FrameId) -> Result<FrameWindowRelation, String> {
        let tree = self.frame_tree();
        let frame = tree
            .frame(frame_id)
            .ok_or_else(|| format!("unknown frame {}", frame_id.get()))?;
        let opener = (frame_id == tree.root_id())
            .then(|| self.frame_windows.opener())
            .flatten();
        Ok(FrameWindowRelation {
            frame_id,
            parent: frame.parent(),
            top: tree.root_id(),
            opener,
        })
    }
}
