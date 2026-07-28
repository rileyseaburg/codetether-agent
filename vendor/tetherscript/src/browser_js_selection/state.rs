use super::*;

#[derive(Clone)]
pub(super) struct RangeBoundary {
    pub handle: DomHandle,
    pub offset: usize,
}

#[derive(Clone)]
pub(super) struct RangeState {
    pub start: RangeBoundary,
    pub end: RangeBoundary,
}

impl RangeState {
    pub fn collapsed(handle: &DomHandle, offset: usize) -> Self {
        Self {
            start: boundary(handle, offset),
            end: boundary(handle, offset),
        }
    }

    pub fn select_node(handle: &DomHandle) -> Self {
        let Some(parent) = handle.parent() else {
            return Self::select_contents(handle);
        };
        let offset = handle.path.last().copied().unwrap_or(0);
        Self {
            start: boundary(&parent, offset),
            end: boundary(&parent, offset.saturating_add(1)),
        }
    }

    pub fn select_contents(handle: &DomHandle) -> Self {
        Self {
            start: boundary(handle, 0),
            end: boundary(handle, extent::node_extent(handle)),
        }
    }

    pub fn is_collapsed(&self) -> bool {
        Rc::ptr_eq(&self.start.handle.root, &self.end.handle.root)
            && self.start.handle.path == self.end.handle.path
            && self.start.offset == self.end.offset
    }
}

pub(super) fn boundary(handle: &DomHandle, offset: usize) -> RangeBoundary {
    RangeBoundary {
        handle: handle.clone(),
        offset,
    }
}
