//! Semantic terminal palette presets and message-role colors.
//!
//! [ColorPalette] supplies the existing dark, light, Solarized and marketing
//! presets. Its default is the marketing palette; role lookup retains each
//! preset's semantic colors. Available with the ratatui feature.
use ratatui::style::Color;
mod dark;
mod default;
mod light;
mod marketing;
mod roles;
mod solarized_dark;
mod solarized_light;

/// Semantic color system shared by terminal components.
/// # Examples
/// ```
/// let palette = codetether_theme::ColorPalette::marketing();
/// assert_eq!(palette.get_message_color("assistant"), palette.assistant_message);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorPalette {
    /// Color for user messages
    pub user_message: Color,
    /// Color for assistant messages
    pub assistant_message: Color,
    /// Color for system messages
    pub system_message: Color,
    /// Color for error states
    pub error: Color,
    /// Color for timestamps
    pub timestamp: Color,
    /// Color for borders and dividers
    pub border: Color,
    /// Color for code blocks
    pub code_block: Color,
    /// Color for tool messages
    pub tool_message: Color,
    /// Default text color
    pub text: Color,
    /// Background color
    pub background: Color,
}
