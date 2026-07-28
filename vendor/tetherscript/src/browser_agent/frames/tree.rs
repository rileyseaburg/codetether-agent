use super::{BrowserFrame, FrameId};

/// Native frame tree with stable frame ids and parent/child traversal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameTree {
    pub(super) frames: Vec<BrowserFrame>,
    pub(super) next_id: u64,
}

impl FrameTree {
    /// Create a tree containing one root frame.
    pub fn new(root_url: impl Into<String>) -> Self {
        let root = BrowserFrame::new(FrameId::new(1), None, root_url.into(), String::new());
        Self {
            frames: vec![root],
            next_id: 2,
        }
    }

    /// Return the root frame id.
    pub fn root_id(&self) -> FrameId {
        self.frames[0].id()
    }

    /// Return the root frame.
    pub fn root(&self) -> &BrowserFrame {
        &self.frames[0]
    }

    /// Return a frame by id.
    pub fn frame(&self, id: FrameId) -> Option<&BrowserFrame> {
        self.frames.iter().find(|frame| frame.id() == id)
    }

    /// Add a child frame under `parent`.
    pub fn add_child(
        &mut self,
        parent: FrameId,
        url: impl Into<String>,
        name: impl Into<String>,
    ) -> Result<FrameId, String> {
        let child = FrameId::new(self.next_id);
        let parent_frame = self
            .frames
            .iter_mut()
            .find(|frame| frame.id() == parent)
            .ok_or_else(|| format!("unknown parent frame {}", parent.get()))?;
        self.next_id += 1;
        parent_frame.add_child(child);
        self.frames.push(BrowserFrame::new(
            child,
            Some(parent),
            url.into(),
            name.into(),
        ));
        Ok(child)
    }
}
