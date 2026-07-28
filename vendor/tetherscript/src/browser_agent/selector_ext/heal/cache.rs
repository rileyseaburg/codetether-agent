//! Selector healing cache.

use super::score::ElementProps;
use std::collections::HashMap;

/// Cache mapping selectors to their previously resolved element fingerprints.
#[derive(Clone, Debug, Default)]
pub struct SelectorHealCache {
    entries: HashMap<String, ElementProps>,
}

impl SelectorHealCache {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    pub fn remember(&mut self, selector: impl Into<String>, props: ElementProps) {
        self.entries.insert(selector.into(), props);
    }
    pub fn get(&self, selector: &str) -> Option<&ElementProps> {
        self.entries.get(selector)
    }
    pub fn remove(&mut self, selector: &str) -> Option<ElementProps> {
        self.entries.remove(selector)
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
