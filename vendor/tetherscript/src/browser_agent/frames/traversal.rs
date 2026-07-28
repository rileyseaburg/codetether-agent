use super::{BrowserFrame, FrameId, FrameTree};

impl FrameTree {
    /// Return all frames in deterministic tree insertion order.
    pub fn frames(&self) -> &[BrowserFrame] {
        &self.frames
    }

    /// Return the parent frame for `id`.
    pub fn parent_of(&self, id: FrameId) -> Option<&BrowserFrame> {
        self.frame(id)
            .and_then(|frame| frame.parent())
            .and_then(|parent| self.frame(parent))
    }

    /// Return child frames for `id` in insertion order.
    pub fn children_of(&self, id: FrameId) -> Vec<&BrowserFrame> {
        self.frame(id).map_or(Vec::new(), |frame| {
            frame
                .children()
                .iter()
                .filter_map(|child| self.frame(*child))
                .collect()
        })
    }
}
