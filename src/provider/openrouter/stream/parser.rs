//! Stateful SSE parsing for OpenRouter streaming completions.

use super::tool_state::ToolState;
use crate::provider::StreamChunk;
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct Parser {
    pub(super) buffer: String,
    pub(super) tools: BTreeMap<usize, ToolState>,
    pub(super) done: bool,
}

impl Parser {
    pub(super) fn push(&mut self, bytes: &[u8]) -> Vec<StreamChunk> {
        self.buffer.push_str(&String::from_utf8_lossy(bytes));
        let mut output = Vec::new();
        while let Some(line_end) = self.buffer.find('\n') {
            let line = self.buffer[..line_end].trim().to_string();
            self.buffer.drain(..=line_end);
            self.parse_line(&line, &mut output);
        }
        output
    }
}
