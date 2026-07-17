//! Default manager construction from the current workspace.

use super::WorktreeManager;
use std::path::PathBuf;

impl Default for WorktreeManager {
    fn default() -> Self {
        Self::new(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }
}
