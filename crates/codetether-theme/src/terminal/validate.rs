//! Adapt configured theme colors to the detected terminal capabilities.
use super::{detect_color_support, fallback::fallback_color};
use crate::Theme;

/// Validate and adjust theme based on terminal capabilities
/// # Arguments
/// * `theme` - Theme to adapt without modifying the original.
/// # Returns
/// A cloned theme whose colors fit the detected terminal capabilities.
/// # Examples
/// ```
/// let theme = codetether_theme::Theme::dark();
/// let _adapted = codetether_theme::terminal::validate_theme(&theme);
/// ```
pub fn validate_theme(theme: &Theme) -> Theme {
    let support = detect_color_support();
    let mut validated = theme.clone();

    if !support.supports_rgb() {
        // Convert RGB colors to closest named colors
        validated.user_color = fallback_color(&theme.user_color, &support);
        validated.assistant_color = fallback_color(&theme.assistant_color, &support);
        validated.system_color = fallback_color(&theme.system_color, &support);
        validated.tool_color = fallback_color(&theme.tool_color, &support);
        validated.error_color = fallback_color(&theme.error_color, &support);
        validated.border_color = fallback_color(&theme.border_color, &support);
        validated.input_border_color = fallback_color(&theme.input_border_color, &support);
        validated.help_border_color = fallback_color(&theme.help_border_color, &support);
        validated.timestamp_color = fallback_color(&theme.timestamp_color, &support);
        validated.code_block_color = fallback_color(&theme.code_block_color, &support);
        validated.status_bar_foreground = fallback_color(&theme.status_bar_foreground, &support);
        validated.status_bar_background = fallback_color(&theme.status_bar_background, &support);

        if let Some(bg) = &theme.background {
            validated.background = Some(fallback_color(bg, &support));
        }
    }

    validated
}
