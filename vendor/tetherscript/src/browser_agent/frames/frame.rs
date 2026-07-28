use super::FrameId;

/// Metadata and tree links for one browser frame.
///
/// The frame stores only native Rust identity, URL/name metadata, and parent
/// child links. Runtime state remains owned by higher-level page/session types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserFrame {
    id: FrameId,
    parent: Option<FrameId>,
    children: Vec<FrameId>,
    url: String,
    name: String,
}

impl BrowserFrame {
    pub(super) fn new(id: FrameId, parent: Option<FrameId>, url: String, name: String) -> Self {
        Self {
            id,
            parent,
            children: Vec::new(),
            url,
            name,
        }
    }

    /// Return this frame's stable identifier.
    pub fn id(&self) -> FrameId {
        self.id
    }

    /// Return the parent frame id, or `None` for the root frame.
    pub fn parent(&self) -> Option<FrameId> {
        self.parent
    }

    /// Return child frame ids in insertion order.
    pub fn children(&self) -> &[FrameId] {
        &self.children
    }

    /// Return this frame's current URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Return this frame's current browsing-context name.
    pub fn name(&self) -> &str {
        &self.name
    }

    pub(super) fn add_child(&mut self, id: FrameId) {
        self.children.push(id);
    }

    pub(super) fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub(super) fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
