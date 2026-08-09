//! In-memory source editing session for a pending patch approval.

use std::path::{Path, PathBuf};

use crate::tui::ui::editor::FileBuffer;

#[path = "edit_session/load.rs"]
mod load;
#[path = "edit_session/patch.rs"]
mod patch;

#[derive(Debug)]
pub(crate) struct ApprovalEditFile {
    path: PathBuf,
    relative: String,
    original: String,
    revised: String,
}

#[derive(Debug)]
pub(crate) struct ApprovalEditSession {
    pub(crate) id: String,
    files: Vec<ApprovalEditFile>,
    index: usize,
}

impl ApprovalEditSession {
    pub(crate) fn from_patch(root: &Path, id: String, patch: &str) -> anyhow::Result<Self> {
        load::session(root, id, patch)
    }

    pub(crate) fn buffer(&self) -> FileBuffer {
        let file = &self.files[self.index];
        FileBuffer::proposed(&file.path, &file.revised)
    }

    pub(crate) fn store(&mut self, text: String) -> bool {
        self.files[self.index].revised = text;
        let more = self.index + 1 < self.files.len();
        if more {
            self.index += 1;
        }
        more
    }

    pub(crate) fn progress(&self) -> (usize, usize) {
        (self.index + 1, self.files.len())
    }

    pub(crate) fn revised_patch(&self) -> String {
        patch::build(&self.files)
    }
}
