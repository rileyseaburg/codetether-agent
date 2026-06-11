//! Unified diff parser for patch hunks.

use super::hunk_builder::HunkBuilder;
use super::types::PatchHunk;

#[derive(Default)]
struct PatchParser {
    current_file: Option<String>,
    current_hunk: Option<HunkBuilder>,
    hunks: Vec<PatchHunk>,
}

/// Parse all valid hunks from a unified diff patch.
pub(super) fn parse_patch(patch: &str) -> Vec<PatchHunk> {
    let mut parser = PatchParser::default();
    for line in patch.lines() {
        parser.absorb(line);
    }
    parser.finish()
}

impl PatchParser {
    fn absorb(&mut self, line: &str) {
        if let Some(file) = file_header(line) {
            self.current_file = Some(file);
        } else if line.starts_with("@@ ") {
            self.start_hunk(line);
        } else if let Some(hunk) = self.current_hunk.as_mut() {
            hunk.absorb(line);
        }
    }

    fn start_hunk(&mut self, line: &str) {
        self.flush_hunk();
        self.current_hunk = HunkBuilder::from_header(line);
    }

    fn finish(mut self) -> Vec<PatchHunk> {
        self.flush_hunk();
        self.hunks
    }

    fn flush_hunk(&mut self) {
        if let (Some(hunk), Some(file)) = (self.current_hunk.take(), &self.current_file) {
            self.hunks.push(hunk.build(file.clone()));
        }
    }
}

fn file_header(line: &str) -> Option<String> {
    let path = line.strip_prefix("+++ ")?;
    let path = path.strip_prefix("b/").unwrap_or(path);
    Some(path.split('\t').next().unwrap_or(path).to_string())
}
