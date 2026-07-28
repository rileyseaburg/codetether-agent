use super::{BrowserFrame, FrameId, FrameTree};

impl FrameTree {
    /// Return the top/root frame for any frame in the tree.
    pub fn top_for(&self, id: FrameId) -> Option<&BrowserFrame> {
        self.frame(id).map(|_| self.root())
    }

    /// Update a frame's current URL.
    ///
    /// # Errors
    ///
    /// Returns an error when `id` does not identify a frame in this tree.
    pub fn set_url(&mut self, id: FrameId, url: impl Into<String>) -> Result<(), String> {
        let frame = self.frame_mut(id)?;
        frame.set_url(url.into());
        Ok(())
    }

    /// Update a frame's browsing-context name.
    ///
    /// # Errors
    ///
    /// Returns an error when `id` does not identify a frame in this tree.
    pub fn set_name(&mut self, id: FrameId, name: impl Into<String>) -> Result<(), String> {
        let frame = self.frame_mut(id)?;
        frame.set_name(name.into());
        Ok(())
    }

    fn frame_mut(&mut self, id: FrameId) -> Result<&mut BrowserFrame, String> {
        self.frames
            .iter_mut()
            .find(|frame| frame.id() == id)
            .ok_or_else(|| format!("unknown frame {}", id.get()))
    }
}
