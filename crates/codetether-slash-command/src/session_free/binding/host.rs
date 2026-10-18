//! Frontend state and view effects for commands without a session.
/// Implementation detail of [crate::declare_frontend_session_free].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_session_free_host {
    ($mode:path) => {
        mod host {
            use super::{App, Arc, Path, ProviderRegistry};
            use $mode as ViewMode;
            pub(super) struct Native<'a> {
                pub app: &'a mut App,
                pub cwd: &'a Path,
                pub registry: &'a Option<Arc<ProviderRegistry>>,
            }
            impl $crate::session_free::Host for Native<'_> {
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
        }
    };
}
