//! Application adapter for composer rendering.
use crate::tui::{app::state::App, color_palette::ColorPalette};
use ratatui::{Frame, layout::Rect};
/// Draw the textbox and its cursor using the current input state.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `app` - Current input state.
/// * `area` - Input rectangle.
/// * `palette` - Border palette.
/// * `suffix` - Input-title suffix.
///
/// # Returns
/// Draws the component without modifying the input state.
///
/// # Examples
/// ```no_run
/// use codetether_agent::tui::ui::chat_view::input_area::render_input;
/// # fn draw(f: &mut ratatui::Frame, app: &codetether_agent::tui::app::state::App) {
/// let area = f.area();
/// let palette = codetether_agent::tui::color_palette::ColorPalette::marketing();
/// render_input(f, app, area, &palette, "");
/// # }
/// ```
pub fn render_input(f: &mut Frame, app: &App, area: Rect, palette: &ColorPalette, suffix: &str) {
    codetether_composer_view::render_input(
        f,
        &super::composer_view::projection(app),
        area,
        palette,
        suffix,
    );
}
