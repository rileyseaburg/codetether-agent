//! Cancellable background runtime for workspace-symbol searches.

mod store;

use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::completion::Completion;
use codetether_lsp_tool::Host;

static EPOCH: AtomicU64 = AtomicU64::new(0);

pub(crate) fn schedule<H: Host>(query: String, files: Vec<PathBuf>, root: PathBuf) {
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
        let result = super::search::workspace::<H>(&files, &query).await;
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

/// Cancels the pending process-wide query and invalidates its epoch.
/// # Arguments
/// Takes no arguments; this runtime serves one interactive TUI.
/// # Returns
/// Unit after aborting the stored task, if any.
/// # Panics
/// Panics if the task-slot mutex is poisoned.
/// # Examples
/// ```
/// codetether_symbol_runtime::cancel();
/// ```
pub fn cancel() {
    EPOCH.fetch_add(1, Ordering::AcqRel);
    store::abort_task();
}

pub(super) fn take() -> Option<Completion> {
    store::take()
}
