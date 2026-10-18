//! Initial file picker state.
use crate::types::*;
use std::path::PathBuf;

impl Default for FilePickerState {
    fn default() -> Self {
        Self {
            active: false,
            workspace_dir: PathBuf::new(),
            dir: PathBuf::new(),
            entries: Vec::new(),
            all_entries: Vec::new(),
            selected: 0,
            filter: String::new(),
            mode: FilePickerMode::Browse,
            preview: None,
            preview_scroll: 0,
        }
    }
}
