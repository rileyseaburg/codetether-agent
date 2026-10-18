//! Global task and result slots for the single interactive TUI.

use std::sync::{Mutex, OnceLock};

use super::Completion;

static TASK: OnceLock<Mutex<Option<tokio::task::JoinHandle<()>>>> = OnceLock::new();
static COMPLETED: OnceLock<Mutex<Option<Completion>>> = OnceLock::new();

pub(super) fn replace_task(task: tokio::task::JoinHandle<()>) {
    *tasks().lock().expect("symbol task lock") = Some(task);
}

pub(super) fn abort_task() {
    if let Some(task) = tasks().lock().expect("symbol task lock").take() {
        task.abort();
    }
}

pub(super) fn complete(result: Completion) {
    *completed().lock().expect("symbol result lock") = Some(result);
}

pub(super) fn take() -> Option<Completion> {
    completed().lock().expect("symbol result lock").take()
}

fn tasks() -> &'static Mutex<Option<tokio::task::JoinHandle<()>>> {
    TASK.get_or_init(|| Mutex::new(None))
}

fn completed() -> &'static Mutex<Option<Completion>> {
    COMPLETED.get_or_init(|| Mutex::new(None))
}
