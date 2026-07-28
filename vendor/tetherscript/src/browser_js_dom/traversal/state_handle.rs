use super::*;

impl TraversalState {
    pub(super) fn handle_at(&mut self, index: usize) -> Option<DomHandle> {
        let path = self.paths.get(index)?.clone();
        self.current = path.clone();
        Some(DomHandle {
            root: self.root.root.clone(),
            path,
        })
    }
}
