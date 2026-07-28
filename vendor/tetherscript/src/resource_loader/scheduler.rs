//! Concurrent, priority-aware load scheduler.
use std::collections::{BinaryHeap, HashMap, HashSet};

use super::ResourceRequest;

#[derive(Debug)]
pub struct LoadScheduler {
    per_origin: usize,
    queue: BinaryHeap<ResourceRequest>,
    active: HashMap<String, usize>,
    pub dns_prefetches: HashSet<String>,
    pub preconnects: HashSet<String>,
}

impl LoadScheduler {
    pub fn new(per_origin: usize) -> Self {
        Self {
            per_origin,
            queue: BinaryHeap::new(),
            active: HashMap::new(),
            dns_prefetches: HashSet::new(),
            preconnects: HashSet::new(),
        }
    }

    pub fn enqueue(&mut self, r: ResourceRequest) {
        self.queue.push(r);
    }

    pub fn add_dns_prefetch(&mut self, url: &str) {
        self.dns_prefetches.insert(origin_of(url));
    }

    pub fn add_preconnect(&mut self, url: &str) {
        self.preconnects.insert(origin_of(url));
    }

    pub fn next_startable(&mut self) -> Option<ResourceRequest> {
        let mut skipped = Vec::new();
        let picked = loop {
            let r = self.queue.pop()?;
            let o = origin_of(&r.url);
            if self.active.get(&o).copied().unwrap_or(0) < self.per_origin {
                *self.active.entry(o).or_default() += 1;
                break Some(r);
            }
            skipped.push(r);
        };
        for r in skipped {
            self.queue.push(r);
        }
        picked
    }

    pub fn finish(&mut self, url: &str) {
        let o = origin_of(url);
        if let Some(n) = self.active.get_mut(&o) {
            *n = n.saturating_sub(1);
        }
    }

    pub fn queued(&self) -> usize {
        self.queue.len()
    }
}

pub fn origin_of(u: &str) -> String {
    let (scheme, rest) = u.split_once("://").unwrap_or(("", u));
    format!("{}://{}", scheme, rest.split('/').next().unwrap_or(rest))
}
