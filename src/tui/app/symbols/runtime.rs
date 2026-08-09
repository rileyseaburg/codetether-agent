//! Cancellable background runtime for workspace-symbol searches.

mod store;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::tui::symbol_search::SymbolEntry;

pub(super) struct Completion {
    pub query: String,
    pub files: Vec<PathBuf>,
    pub result: anyhow::Result<Vec<SymbolEntry>>,
}

static EPOCH: AtomicU64 = AtomicU64::new(0);

pub(super) fn schedule(query: String, files: Vec<PathBuf>, root: PathBuf) {
    cancel();
    let epoch = EPOCH.load(Ordering::Acquire);
    let task = tokio::spawn(async move {
        tokio::time::sleep(super::gate::DEBOUNCE).await;
        if EPOCH.load(Ordering::Acquire) != epoch {
            return;
        }
        let files = if files.is_empty() {
            super::languages::representatives(&root)
        } else {
            files
        };
        let result = super::search::workspace(&files, &query).await;
        if EPOCH.load(Ordering::Acquire) == epoch {
            store::complete(Completion {
                query,
                files,
                result,
            });
        }
    });
    store::replace_task(task);
}

pub(super) fn cancel() {
    EPOCH.fetch_add(1, Ordering::AcqRel);
    store::abort_task();
}

pub(super) fn take() -> Option<Completion> {
    store::take()
}
