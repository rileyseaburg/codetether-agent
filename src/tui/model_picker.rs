//! Application projection for the shared model picker.
use crate::tui::app::{session_runtime::SessionView, state::App};
use ratatui::{Frame, layout::Rect};
pub fn render_model_picker(f: &mut Frame, area: Rect, app: &mut App, session: &SessionView) {
    let view = codetether_model_picker::PickerView {
        catalog: &app.state.model_picker.available_models,
        filter: &app.state.model_picker.model_filter,
        status: &app.state.status,
        selected_index: app.state.model_picker.selected_model_index,
        refreshing: app.state.model_refresh_in_flight,
        session_model: session.model.as_deref(),
    };
    codetether_model_picker::render_model_picker(f, area, &view);
}
