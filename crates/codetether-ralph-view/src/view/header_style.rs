//! Ralph header style presentation.
use crate::RalphViewState;
use ratatui::style::Color;
pub(super) fn style(state: &RalphViewState, failed: usize) -> (&'static str, Color) {
    let title = if state.complete {
        if state.error.is_some() {
            " Ralph [ERROR] "
        } else if failed > 0 {
            " Ralph [PARTIAL] "
        } else {
            " Ralph [COMPLETE] "
        }
    } else {
        " Ralph [ACTIVE] "
    };

    let border_color = if state.complete {
        if state.error.is_some() || failed > 0 {
            Color::Red
        } else {
            Color::Green
        }
    } else {
        Color::Magenta
    };

    (title, border_color)
}
