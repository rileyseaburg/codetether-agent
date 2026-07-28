//! Load state tracking and aggregate progress.
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadStatus {
    Pending,
    Loading,
    Loaded,
    Error,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LoadProgress {
    pub loaded: u64,
    pub total: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct LoadEntry {
    pub status: LoadStatus,
    pub progress: LoadProgress,
}

#[derive(Debug, Default)]
pub struct LoadStateTracker {
    entries: HashMap<String, LoadEntry>,
}

impl LoadStateTracker {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    pub fn insert(&mut self, url: String, status: LoadStatus) {
        self.entries
            .entry(url)
            .and_modify(|e| e.status = status)
            .or_insert(LoadEntry {
                status,
                progress: LoadProgress::default(),
            });
    }

    pub fn set_progress(&mut self, url: &str, loaded: u64, total: Option<u64>) {
        self.entries
            .entry(url.into())
            .and_modify(|e| e.progress = LoadProgress { loaded, total })
            .or_insert(LoadEntry {
                status: LoadStatus::Loading,
                progress: LoadProgress { loaded, total },
            });
    }

    pub fn get(&self, url: &str) -> Option<&LoadEntry> {
        self.entries.get(url)
    }

    pub fn progress(&self) -> LoadProgress {
        let loaded = self.entries.values().map(|e| e.progress.loaded).sum();
        let mut known = true;
        let total = self
            .entries
            .values()
            .map(|e| {
                known &= e.progress.total.is_some();
                e.progress.total.unwrap_or(0)
            })
            .sum();
        LoadProgress {
            loaded,
            total: known.then_some(total),
        }
    }
}
