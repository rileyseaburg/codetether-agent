//! ResizeObserver with size-change detection.

use super::resize_types::ResizeEntry;
use std::collections::HashMap;

pub type ResizeCallback = Box<dyn FnMut(Vec<ResizeEntry>)>;

pub struct ResizeObserver {
    targets: HashMap<u64, Option<(i64, i64)>>,
    callback: ResizeCallback,
}

impl ResizeObserver {
    pub fn new(callback: ResizeCallback) -> Self {
        Self {
            targets: HashMap::new(),
            callback,
        }
    }
    pub fn observe(&mut self, id: u64) {
        self.targets.entry(id).or_insert(None);
    }
    pub fn unobserve(&mut self, id: u64) {
        self.targets.remove(&id);
    }
    pub fn disconnect(&mut self) {
        self.targets.clear();
    }
    pub fn check<F: Fn(u64) -> Option<(i64, i64)>>(&mut self, size_of: F) {
        let mut entries = vec![];
        for (&id, last) in self.targets.iter_mut() {
            if let Some(size) = size_of(id) {
                if last.is_none_or(|s| s != size) {
                    *last = Some(size);
                    entries.push(ResizeEntry {
                        target_node_id: id,
                        content_width: size.0,
                        content_height: size.1,
                    });
                }
            }
        }
        if !entries.is_empty() {
            (self.callback)(entries);
        }
    }
}
