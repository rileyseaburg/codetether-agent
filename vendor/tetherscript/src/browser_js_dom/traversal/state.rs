use super::*;

#[derive(Clone, Copy)]
pub(super) enum TraversalKind {
    TreeWalker,
    NodeIterator,
}

pub(super) struct TraversalState {
    pub(super) root: DomHandle,
    pub(super) paths: Vec<Vec<usize>>,
    pub(super) current: Vec<usize>,
    pub(super) cursor: usize,
    pub(super) kind: TraversalKind,
}

impl TraversalState {
    pub(super) fn new(root: DomHandle, paths: Vec<Vec<usize>>, kind: TraversalKind) -> Self {
        Self {
            current: root.path.clone(),
            root,
            paths,
            cursor: 0,
            kind,
        }
    }
}
