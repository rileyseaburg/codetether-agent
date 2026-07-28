use super::FrameTree;
use crate::browser::Document;

impl FrameTree {
    /// Build frame metadata from iframe elements in `document`.
    ///
    /// The root frame uses `root_url`. Child ids are deterministic from DOM
    /// paths, so repeated scans of the same iframe order keep stable ids.
    pub fn from_document(root_url: impl Into<String>, document: &Document) -> Self {
        let mut tree = Self::new(root_url);
        let mut path = Vec::new();
        super::walk::collect_frames(&document.children, tree.root_id(), &mut path, &mut tree);
        tree
    }
}
