//! Builder for one unified-diff hunk.

use super::types::PatchHunk;

#[path = "hunk_body.rs"]
mod body;
#[path = "hunk_range.rs"]
mod range;

/// Incrementally collects old and new hunk lines.
pub(super) struct HunkBuilder {
    start_line: usize,
    counts: (usize, usize),
    old_lines: Vec<String>,
    new_lines: Vec<String>,
}

impl HunkBuilder {
    pub(super) fn from_header(line: &str) -> Option<Self> {
        let (start_line, old_count) = range::parse(line.split_whitespace().nth(1)?, '-')?;
        let (_, new_count) = range::parse(line.split_whitespace().nth(2)?, '+')?;
        Some(Self {
            start_line,
            counts: (old_count, new_count),
            old_lines: Vec::new(),
            new_lines: Vec::new(),
        })
    }

    pub(super) fn absorb(&mut self, line: &str) {
        body::absorb(&mut self.old_lines, &mut self.new_lines, line);
    }

    pub(super) fn complete(&self) -> bool {
        self.counts == (self.old_lines.len(), self.new_lines.len())
    }

    pub(super) fn build(self, file: String) -> PatchHunk {
        PatchHunk {
            file,
            start_line: self.start_line,
            old_lines: self.old_lines,
            new_lines: self.new_lines,
        }
    }
}
