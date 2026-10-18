//! Terminal color capability levels and supported representations.

/// Color representations available in a terminal.
/// # Examples
/// ```
/// use codetether_theme::terminal::ColorSupport;
/// let support = ColorSupport::TrueColor;
/// assert!(support.supports_rgb());
/// match support { ColorSupport::Monochrome | ColorSupport::Ansi8 | ColorSupport::Ansi256 | ColorSupport::TrueColor => {} }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ColorSupport {
    /// No color capability.
    Monochrome,
    /// Basic named ANSI colors.
    Ansi8,
    /// Indexed ANSI palette.
    Ansi256,
    /// Full RGB color support.
    TrueColor,
}

impl ColorSupport {
    /// Whether the terminal can display RGB colors.
    pub fn supports_rgb(&self) -> bool {
        matches!(self, ColorSupport::TrueColor)
    }

    /// Whether the terminal supports indexed ANSI colors.
    pub fn supports_indexed(&self) -> bool {
        matches!(self, ColorSupport::Ansi256 | ColorSupport::TrueColor)
    }

    /// Whether the terminal supports named colors.
    pub fn supports_named(&self) -> bool {
        !matches!(self, ColorSupport::Monochrome)
    }
}
