//! Inline-level box and text run types.

/// Vertical alignment for inline boxes inside a line box.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
    Baseline,
}

impl Default for VerticalAlign {
    fn default() -> Self {
        Self::Baseline
    }
}

/// A contiguous run of text with a single style.
#[derive(Clone, Debug, PartialEq)]
pub struct TextRun {
    pub text: String,
    pub font_size: f32,
}

impl TextRun {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: crate::inline_layout::measure::DEFAULT_FONT_SIZE,
        }
    }

    pub fn with_font_size(text: impl Into<String>, font_size: f32) -> Self {
        Self {
            text: text.into(),
            font_size,
        }
    }
}

/// The payload of an inline-level box.
#[derive(Clone, Debug, PartialEq)]
pub enum InlineBoxKind {
    Text(TextRun),
    InlineBlock { width: f32, height: f32 },
    Image { width: f32, height: f32 },
    LineBreak,
}

/// An inline-level element: text, atomic inline content, image, or `<br>`.
#[derive(Clone, Debug, PartialEq)]
pub struct InlineBox {
    pub kind: InlineBoxKind,
    pub vertical_align: VerticalAlign,
}

impl InlineBox {
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            kind: InlineBoxKind::Text(TextRun::new(text)),
            vertical_align: VerticalAlign::Baseline,
        }
    }

    pub fn text_run(run: TextRun) -> Self {
        Self {
            kind: InlineBoxKind::Text(run),
            vertical_align: VerticalAlign::Baseline,
        }
    }

    pub fn inline_block(width: f32, height: f32) -> Self {
        Self {
            kind: InlineBoxKind::InlineBlock { width, height },
            vertical_align: VerticalAlign::Baseline,
        }
    }

    pub fn image(width: f32, height: f32) -> Self {
        Self {
            kind: InlineBoxKind::Image { width, height },
            vertical_align: VerticalAlign::Baseline,
        }
    }

    pub fn br() -> Self {
        Self {
            kind: InlineBoxKind::LineBreak,
            vertical_align: VerticalAlign::Baseline,
        }
    }

    pub fn with_vertical_align(mut self, align: VerticalAlign) -> Self {
        self.vertical_align = align;
        self
    }
}
