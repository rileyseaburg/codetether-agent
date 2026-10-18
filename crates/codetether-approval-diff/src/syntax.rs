//! Stateful syntax highlighting for old and new sides of a unified diff.

use ratatui::text::Span;
use syntect::easy::HighlightLines;

pub(super) struct SyntaxPair<'a> {
    old: HighlightLines<'a>,
    new: HighlightLines<'a>,
}

impl SyntaxPair<'static> {
    pub(super) fn for_path(path: &str) -> Option<Self> {
        let syntax = crate::assets::syntax(path)?;
        Some(Self {
            old: HighlightLines::new(syntax, crate::assets::theme()),
            new: HighlightLines::new(syntax, crate::assets::theme()),
        })
    }

    pub(super) fn context(&mut self, code: &str) -> Vec<Span<'static>> {
        let rendered = crate::highlight::render(&mut self.new, code);
        let _ = self.old.highlight_line(code, crate::assets::syntaxes());
        rendered
    }

    pub(super) fn addition(&mut self, code: &str) -> Vec<Span<'static>> {
        crate::highlight::render(&mut self.new, code)
    }

    pub(super) fn deletion(&mut self, code: &str) -> Vec<Span<'static>> {
        crate::highlight::render(&mut self.old, code)
    }
}
