//! Formatting flags and buffered text for one inline pass.
use ratatui::{style::Color, text::Span};
pub(crate) struct InlineState {
    pub(crate) spans: Vec<Span<'static>>,
    pub(crate) current: String,
    pub(crate) in_bold: bool,
    pub(crate) in_italic: bool,
    pub(crate) in_code: bool,
    pub(crate) role_color: Color,
}
impl InlineState {
    pub(crate) fn new(role: &str) -> Self {
        Self {
            spans: Vec::new(),
            current: String::new(),
            in_bold: false,
            in_italic: false,
            in_code: false,
            role_color: match role {
                "user" => Color::White,
                "assistant" => Color::Cyan,
                "system" | "developer" => Color::Yellow,
                "tool" => Color::Green,
                _ => Color::White,
            },
        }
    }
}
