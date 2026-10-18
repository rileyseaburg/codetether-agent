//! Native attachments and view changes for session-free command dispatch.
use crate::{
    provider::ProviderRegistry,
    tui::{app::state::App, models::ViewMode},
};
use std::{path::Path, sync::Arc};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub registry: &'a Option<Arc<ProviderRegistry>>,
}
impl codetether_slash_command::session_free::Host for Native<'_> {
    fn attach_file(&mut self, path: &str) {
        super::helpers::attach_file_command(self.app, self.cwd, path);
    }
    fn attach_image(&mut self, path: &str) {
        super::image::attach_image_command(self.app, self.cwd, path);
    }
    fn open_settings(&mut self) {
        self.app.state.set_view_mode(ViewMode::Settings);
    }
    fn open_model(&mut self) {
        super::helpers::open_model_view(self.app, self.registry);
    }
    fn open_help(&mut self) {
        self.app.state.show_help = true;
        self.app.state.help_scroll.offset = 0;
        self.app.state.status = "Help".to_string();
    }
}
