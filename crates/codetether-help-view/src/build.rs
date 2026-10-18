//! Help build content and presentation.
use crate::HelpContext;
use codetether_theme::Theme;
use codetether_theme::terminal::{detect_color_support, validate_theme};
use codetether_token_display::TokenDisplay;
use ratatui::text::Line;
/// Build help reference rows using current usage metrics and host labels.
/// # Arguments
/// * `context` - Session, directory, settings and worker labels.
/// # Returns
/// Owned rows in the original help-section order.
/// # Panics
/// Panics if a session ID longer than 20 bytes is not UTF-8 aligned at byte 20.
/// # Examples
/// ```no_run
/// # fn example(context: &codetether_help_view::HelpContext<'_>) {
/// let lines = codetether_help_view::build_help_lines(context);
/// assert!(!lines.is_empty());
/// # }
/// ```
pub fn build_help_lines(context: &HelpContext<'_>) -> Vec<Line<'static>> {
    let token_display = TokenDisplay::new();
    let color_support = detect_color_support();
    let validated_theme = validate_theme(&Theme::default());
    let session_label = context
        .session_id
        .as_deref()
        .map(|id| {
            if id.len() > 20 {
                format!("{}…", &id[..20])
            } else {
                id.to_string()
            }
        })
        .unwrap_or_else(|| "(none)".to_string());

    let mut lines: Vec<Line<'static>> = Vec::with_capacity(80);

    crate::title::append(&mut lines, &token_display);
    crate::session_identity::append(&mut lines, context, session_label);
    crate::session_colors::append(&mut lines, color_support, &validated_theme);
    crate::session_auto_apply::append(&mut lines, context);
    crate::session_network::append(&mut lines, context);
    crate::session_completion::append(&mut lines, context);
    crate::worker::append(&mut lines, context);
    crate::tokens::append(&mut lines, &token_display);
    crate::keyboard::append(&mut lines);
    crate::images::append(&mut lines);
    crate::text_editing::append(&mut lines);
    crate::scrolling::append(&mut lines);
    crate::commands::append(&mut lines);
    crate::navigation_commands::append(&mut lines);
    crate::conversation_commands::append(&mut lines);
    crate::observability::append(&mut lines);
    crate::development::append(&mut lines);
    crate::session_picker::append(&mut lines);
    crate::settings::append(&mut lines);
    crate::bus::append(&mut lines);
    crate::agents::append(&mut lines);

    lines
}
