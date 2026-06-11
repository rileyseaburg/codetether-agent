//! Builder for one unified-diff hunk.

use super::types::PatchHunk;

/// Incrementally collects old and new hunk lines.
pub(super) struct HunkBuilder {
    start_line: usize,
    old_lines: Vec<String>,
    new_lines: Vec<String>,
}

impl HunkBuilder {
    pub(super) fn from_header(line: &str) -> Option<Self> {
        let old_range = line.split_whitespace().nth(1)?.strip_prefix('-')?;
        let start_line = old_range
            .split(',')
            .next()
            .and_then(|value| value.parse().ok())
            .unwrap_or(1);
        Some(Self {
            start_line,
            old_lines: Vec::new(),
            new_lines: Vec::new(),
        })
    }

    pub(super) fn absorb(&mut self, line: &str) {
        if let Some(stripped) = line.strip_prefix('-') {
            self.old_lines.push(stripped.to_string());
        } else if let Some(stripped) = line.strip_prefix('+') {
            self.new_lines.push(stripped.to_string());
        } else if line.starts_with(' ') || line.is_empty() {
            self.push_context(line);
        }
    }

    pub(super) fn build(self, file: String) -> PatchHunk {
        PatchHunk {
            file,
            start_line: self.start_line,
            old_lines: self.old_lines,
            new_lines: self.new_lines,
        }
    }

    fn push_context(&mut self, line: &str) {
        let content = if line.is_empty() { "" } else { &line[1..] };
        self.old_lines.push(content.to_string());
        self.new_lines.push(content.to_string());
    }
}
