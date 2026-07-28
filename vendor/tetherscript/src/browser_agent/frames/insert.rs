use super::{BrowserFrame, FrameId, FrameTree};

impl FrameTree {
    pub(super) fn add_child_with_id(
        &mut self,
        parent: FrameId,
        id: FrameId,
        url: String,
        name: String,
    ) -> Result<(), String> {
        if self.frame(id).is_some() {
            return Err(format!("duplicate frame id {}", id.get()));
        }
        let parent_frame = self
            .frames
            .iter_mut()
            .find(|frame| frame.id() == parent)
            .ok_or_else(|| format!("unknown parent frame {}", parent.get()))?;
        parent_frame.add_child(id);
        self.next_id = self.next_id.max(id.get().saturating_add(1));
        self.frames
            .push(BrowserFrame::new(id, Some(parent), url, name));
        Ok(())
    }
}
