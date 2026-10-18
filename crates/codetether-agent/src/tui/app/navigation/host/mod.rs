//! Bind shared navigation decisions to the native application.
use crate::tui::app::state::App;
use codetether_tui_navigation::{Action, Context, Host};
mod agents;
mod editing;
mod lists;
mod overlays;
mod page_scroll;
mod pane_scroll;
pub(crate) struct Navigation<'a>(pub &'a mut App);
impl Host for Navigation<'_> {
    fn context(&self) -> Context {
        let app = &*self.0;
        Context {
            view_mode: app.state.view.mode.clone(),
            show_help: app.state.help.visible,
            symbols_active: crate::tui::app::symbols::symbol_search_active(app),
            model_picker_active: app.state.model_picker.model_picker_active,
            swarm_detail: app.state.swarm.detail_mode,
            ralph_detail: app.state.ralph.detail_mode,
            bus_filter: app.state.bus_log.filter_input_mode,
            bus_detail: app.state.bus_log.detail_mode,
            slash_navigable: app
                .state
                .composer
                .slash_suggestions_navigable(&app.state.view.mode, app.state.processing),
            subagent_detail: app.state.agent_tree.detail_mode,
            subagent_scroll: usize::from(app.state.agent_tree.detail_scroll),
            protocol_scroll: app.state.protocol_registry.scroll,
        }
    }
    fn apply(&mut self, action: Action) {
        if overlays::apply(self.0, &action)
            || lists::apply(self.0, &action)
            || agents::apply(self.0, &action)
            || pane_scroll::apply(self.0, &action)
            || page_scroll::apply(self.0, &action)
            || editing::apply(self.0, &action)
        {
            return;
        }
        unreachable!("unmapped navigation action: {action:?}");
    }
    fn file_picker_escape(&mut self) -> bool {
        crate::tui::app::file_picker::file_picker_escape(&mut self.0.state.file_picker)
    }
}

mod focus;
mod tab;
