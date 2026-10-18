//! Shared body renderer for list and detail modes.

use ratatui::{Frame, layout::Rect};

use super::{render_detail, render_footer, render_list, state::BusLogState};

pub(super) fn render_bus_body(
    f: &mut Frame,
    state: &mut BusLogState,
    main_area: Rect,
    footer_area: Rect,
) {
    if state.detail_mode {
        render_detail::render_detail(f, state, main_area);
    } else {
        render_list::render_list(f, state, main_area);
    }
    render_footer::render_footer(f, footer_area);
}
