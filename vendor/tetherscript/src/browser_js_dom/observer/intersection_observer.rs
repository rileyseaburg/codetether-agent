//! IntersectionObserver with threshold-crossing detection.

use super::intersection_compute::{compute_entry, Rect};
use super::intersection_types::IntersectionEntry;
mod threshold;
use std::collections::HashMap;

pub type IntersectionCallback = Box<dyn FnMut(Vec<IntersectionEntry>)>;

pub struct IntersectionObserver {
    thresholds: Vec<f64>,
    targets: HashMap<u64, f64>,
    callback: IntersectionCallback,
}

impl IntersectionObserver {
    pub fn new(thresholds: Vec<f64>, callback: IntersectionCallback) -> Self {
        let mut t: Vec<f64> = if thresholds.is_empty() {
            vec![0.0]
        } else {
            thresholds.into_iter().map(|v| v.clamp(0.0, 1.0)).collect()
        };
        t.sort_by(|a, b| a.partial_cmp(b).unwrap());
        t.dedup();
        Self {
            thresholds: t,
            targets: HashMap::new(),
            callback,
        }
    }
    pub fn observe(&mut self, id: u64) {
        self.targets.entry(id).or_insert(-1.0);
    }
    pub fn unobserve(&mut self, id: u64) {
        self.targets.remove(&id);
    }
    pub fn disconnect(&mut self) {
        self.targets.clear();
    }
    pub fn check<F: Fn(u64) -> Option<Rect>>(&mut self, root: Rect, bounds: F) {
        let mut entries = vec![];
        for (&id, last) in self.targets.iter_mut() {
            if let Some(rect) = bounds(id) {
                let entry = compute_entry(id, root, rect);
                if threshold::crossed(*last, entry.intersection_ratio, &self.thresholds) {
                    *last = entry.intersection_ratio;
                    entries.push(entry);
                }
            }
        }
        if !entries.is_empty() {
            (self.callback)(entries);
        }
    }
}
