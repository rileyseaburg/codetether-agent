//! Compatibility test adapter for shared session-specific todo storage.
use codetether_todo::storage::{ROOT_FILE, load, resolve, save};
#[cfg(test)]
#[path = "todo_path_tests.rs"]
mod tests;
