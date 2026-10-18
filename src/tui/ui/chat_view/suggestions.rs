//! Application adapter for command suggestion rendering.
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};
/// Draw command matches or the current command usage hint.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `app` - Command matches, selection and usage hint.
/// * `area` - Suggestion rectangle.
///
/// # Returns
/// Draws the component without modifying the input state.
///
/// # Examples
/// ```no_run
/// use codetether_agent::tui::ui::chat_view::suggestions::render_suggestions;
/// # fn draw(f: &mut ratatui::Frame, app: &codetether_agent::tui::app::state::App) {
/// let area = f.area();
/// render_suggestions(f, app, area);
/// # }
/// ```
pub fn render_suggestions(f: &mut Frame, app: &App, area: Rect) {
    codetether_composer_view::render_suggestions(
        f,
        &app.state.completion.slash_suggestions,
        app.state.completion.selected_slash_suggestion,
        area,
        || app.state.current_slash_hint(),
    );
}
