//! Track the current file and counted hunk during unified-diff parsing.

use super::super::{hunk_builder::HunkBuilder, types::PatchHunk};

#[path = "parser_header.rs"]
mod header;

#[derive(Default)]
pub(super) struct PatchParser {
    current_file: Option<String>,
    current_hunk: Option<HunkBuilder>,
    hunks: Vec<PatchHunk>,
}

impl PatchParser {
    pub(super) fn absorb(&mut self, line: &str) {
        let complete = self
            .current_hunk
            .as_ref()
            .is_some_and(HunkBuilder::complete);
        if complete {
            self.flush_hunk();
        }
        if line.starts_with("@@ ") {
            self.flush_hunk();
            self.current_hunk = HunkBuilder::from_header(line);
        } else if let Some(hunk) = self.current_hunk.as_mut() {
            hunk.absorb(line);
        } else if let Some(file) = header::file(line) {
            self.current_file = Some(file);
        }
    }

    pub(super) fn finish(mut self) -> Vec<PatchHunk> {
        self.flush_hunk();
        self.hunks
    }

    fn flush_hunk(&mut self) {
        if let (Some(hunk), Some(file)) = (self.current_hunk.take(), &self.current_file) {
            self.hunks.push(hunk.build(file.clone()));
        }
    }
}
