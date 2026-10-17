//! Shared helpers for memory tool tests.

use super::MemoryTool;
use std::sync::atomic::Ordering;

pub fn initialized_tool() -> MemoryTool {
    let tool = MemoryTool::new();
    tool.initialized.store(true, Ordering::SeqCst);
    tool
}

pub fn isolate_data_dir() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    // SAFETY: test-only; each test gets a unique temp path.
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", dir.path()) };
    dir
}
