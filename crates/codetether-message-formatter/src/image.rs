//! Image URL placeholder presentation.
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};
impl crate::MessageFormatter {
    /// Format an image as a simple placeholder line
    /// Format an image URL as a terminal placeholder.
    /// # Arguments
    /// * `url` - Source image URL.
    /// * `_mime_type` - Reserved MIME-type hint.
    /// # Returns
    /// An owned placeholder line.
    /// # Examples
    /// ```
    /// let f = codetether_message_formatter::MessageFormatter::new(80); let _line = f.format_image("image.png", None);
    /// ```
    pub fn format_image(&self, url: &str, _mime_type: Option<&str>) -> Line<'static> {
        // Extract filename from URL for display
        let path = url.split('?').next().unwrap_or(url);
        let filename = path.rsplit('/').next().unwrap_or("image");

        Line::from(vec![
            Span::styled("  🖼️  ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("[Image: {}]", filename),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::ITALIC),
            ),
        ])
    }
}
