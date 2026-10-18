//! Buffers retained while scanning a Markdown message.
use ratatui::text::Line;
#[derive(Default)]
pub(crate) struct ContentState {
    pub(crate) lines: Vec<Line<'static>>,
    pub(crate) in_code_block: bool,
    pub(crate) code_block_start: bool,
    pub(crate) code_block_language: String,
    pub(crate) code_block_lines: Vec<String>,
    pub(crate) in_math_block: bool,
    pub(crate) math_delim: &'static str,
    pub(crate) math_block_lines: Vec<String>,
    pub(crate) table_buf: Vec<String>,
}
