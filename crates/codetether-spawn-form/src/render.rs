//! Drawing the active spawn form overlay.
//! Form state and terminal presentation require no application runtime.

use crate::{SpawnFormState, widget::build_widget};
use codetether_help_view::centered_rect;
use ratatui::{Frame, widgets::Clear};

/// Render the spawn form if it is active.
///
/// # Arguments
/// * `f` — Destination terminal frame.
/// * `form` — Active form, or `None` when closed.
///
/// # Returns
/// Draws the overlay only when a form is present.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame) {
/// codetether_spawn_form::render_spawn_form_if_needed(f, None);
/// # }
/// ```
pub fn render_spawn_form_if_needed(f: &mut Frame, form: Option<&SpawnFormState>) {
    let Some(form) = form else {
        return;
    };
    let area = centered_rect(50, 40, f.area());
    f.render_widget(Clear, area);
    let form = form.clone();
    let widget = build_widget(&form);
    f.render_widget(widget, area);
}
