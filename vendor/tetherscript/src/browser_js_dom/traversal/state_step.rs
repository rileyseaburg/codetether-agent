use super::*;
use std::cmp::Ordering;

impl TraversalState {
    pub(super) fn next(&mut self) -> Option<DomHandle> {
        match self.kind {
            TraversalKind::NodeIterator => self.iterator_next(),
            TraversalKind::TreeWalker => {
                let current = self.current.clone();
                let index = self
                    .paths
                    .iter()
                    .position(|path| cmp_path(path, &current) == Ordering::Greater)?;
                self.handle_at(index)
            }
        }
    }

    pub(super) fn previous(&mut self) -> Option<DomHandle> {
        match self.kind {
            TraversalKind::NodeIterator => self.iterator_previous(),
            TraversalKind::TreeWalker => {
                let current = self.current.clone();
                let index = self
                    .paths
                    .iter()
                    .rposition(|path| cmp_path(path, &current) == Ordering::Less)?;
                self.handle_at(index)
            }
        }
    }

    fn iterator_next(&mut self) -> Option<DomHandle> {
        if self.cursor >= self.paths.len() {
            return None;
        }
        let index = self.cursor;
        self.cursor += 1;
        self.handle_at(index)
    }

    fn iterator_previous(&mut self) -> Option<DomHandle> {
        self.cursor = self.cursor.checked_sub(1)?;
        self.handle_at(self.cursor)
    }
}
