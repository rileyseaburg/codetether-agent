//! Stateful syntax highlighting for old and new sides of a unified diff.

#[rustfmt::skip]
use ratatui::{style::{Color, Style}, text::Span};
use syntect::easy::HighlightLines;

pub(super) struct SyntaxPair<'a> {
    old: HighlightLines<'a>,
    new: HighlightLines<'a>,
}

impl SyntaxPair<'static> {
    pub(super) fn for_path(path: &str) -> Option<Self> {
        let syntax = super::approval_diff_assets::syntax(path)?;
        Some(Self {
            old: HighlightLines::new(syntax, super::approval_diff_assets::theme()),
            new: HighlightLines::new(syntax, super::approval_diff_assets::theme()),
        })
    }

    pub(super) fn context(&mut self, code: &str) -> Vec<Span<'static>> {
        let rendered = render(&mut self.new, code);
        let _ = self
            .old
            .highlight_line(code, super::approval_diff_assets::syntaxes());
        rendered
    }

    pub(super) fn addition(&mut self, code: &str) -> Vec<Span<'static>> {
        render(&mut self.new, code)
    }

    pub(super) fn deletion(&mut self, code: &str) -> Vec<Span<'static>> {
        render(&mut self.old, code)
    }
}

fn render(highlighter: &mut HighlightLines<'_>, code: &str) -> Vec<Span<'static>> {
    highlighter
        .highlight_line(code, super::approval_diff_assets::syntaxes())
        .unwrap_or_default()
        .into_iter()
        .map(|(style, text)| {
            Span::styled(
                text.to_string(),
                Style::default().fg(Color::Rgb(
                    style.foreground.r,
                    style.foreground.g,
                    style.foreground.b,
                )),
            )
        })
        .collect()
}
