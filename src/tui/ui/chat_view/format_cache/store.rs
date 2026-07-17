//! Byte-accounted storage for formatted terminal lines.

use std::collections::HashMap;

use ratatui::text::Line;

use super::{Key, policy};

const ENTRY_LIMIT: usize = 128;

pub(super) struct FormatCache {
    entries: HashMap<Key, Vec<Line<'static>>>,
    bytes: usize,
}

impl FormatCache {
    pub(super) fn new() -> Self {
        Self {
            entries: HashMap::with_capacity(ENTRY_LIMIT),
            bytes: 0,
        }
    }

    pub(super) fn get(&self, key: &Key) -> Option<&Vec<Line<'static>>> {
        self.entries.get(key)
    }

    pub(super) fn insert(&mut self, key: Key, lines: Vec<Line<'static>>) {
        let weight = policy::weight(&lines, lines.capacity());
        if weight > policy::BYTE_LIMIT {
            return;
        }
        if self.entries.len() >= ENTRY_LIMIT
            || self.bytes.saturating_add(weight) > policy::BYTE_LIMIT
        {
            self.clear();
        }
        self.bytes += weight;
        if let Some(previous) = self.entries.insert(key, lines) {
            self.bytes = self
                .bytes
                .saturating_sub(policy::weight(&previous, previous.capacity()));
        }
    }

    pub(super) fn clear(&mut self) {
        self.entries.clear();
        self.bytes = 0;
    }
}

#[cfg(test)]
#[path = "store_tests.rs"]
mod tests;
