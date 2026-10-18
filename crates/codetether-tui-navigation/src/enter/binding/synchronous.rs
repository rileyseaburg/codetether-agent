//! Frontend synchronous Enter effects.
/// Implementation detail of [crate::declare_frontend_enter].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_enter_synchronous {
    ($mode:path; $file:path, $bus:path, $model:path, $subagents:path) => {
        mod synchronous {
            use super::Native;
            use $mode as ViewMode;
            impl $crate::enter::Sync for Native<'_> {
                fn view(&self) -> ViewMode {
                    self.app.state.view_mode.clone()
                }
                fn bus_filter_active(&self) -> bool {
                    self.app.state.bus_log.filter_input_mode
                }
                fn file_picker(&mut self) {
                    $file(self.app, self.cwd);
                }
                fn ralph(&mut self) {
                    self.app.state.ralph.enter_detail();
                }
                fn bus_filter(&mut self) {
                    $bus(self.app);
                }
                fn bus_detail(&mut self) {
                    self.app.state.bus_log.enter_detail();
                }
                fn model(&mut self) {
                    $model(self.app, self.slot);
                }
                fn subagents(&mut self) {
                    $subagents(self.app);
                }
            }
        }
    };
}
