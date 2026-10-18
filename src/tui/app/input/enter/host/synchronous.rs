use super::Native;
use crate::tui::models::ViewMode;
impl codetether_tui_navigation::enter::Sync for Native<'_> {
    fn view(&self) -> ViewMode {
        self.app.state.view_mode.clone()
    }
    fn bus_filter_active(&self) -> bool {
        self.app.state.bus_log.filter_input_mode
    }
    fn file_picker(&mut self) {
        crate::tui::app::file_picker::file_picker_enter(self.app, self.cwd);
    }
    fn ralph(&mut self) {
        self.app.state.ralph.enter_detail();
    }
    fn bus_filter(&mut self) {
        super::super::super::bus::handle_enter_bus_filter(self.app);
    }
    fn bus_detail(&mut self) {
        self.app.state.bus_log.enter_detail();
    }
    fn model(&mut self) {
        super::super::super::model_apply::selected(self.app, self.slot);
    }
    fn subagents(&mut self) {
        super::super::super::enter_subagents::dispatch(self.app);
    }
}
