//! Frontend composer state for image-first paste ingestion.
/// Implementation detail of [crate::declare_frontend_paste].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_paste_input {
    ($app:path, $mode:path, $attach:path) => {
        mod host {
            use $app as App;
            use $crate::PendingTextPaste;
            use $mode as InputMode;
            impl $crate::input::Host for App {
                fn try_attach_image(&mut self, text: &str) -> bool {
                    $attach(self, text)
                }
                fn input(&self) -> &str {
                    &self.state.composer.input
                }
                fn set_command_mode(&mut self, command: bool) {
                    self.state.composer.mode = if command {
                        InputMode::Command
                    } else {
                        InputMode::Editing
                    };
                }
                fn insert_text(&mut self, text: &str) {
                    self.state.composer.insert_text(text);
                }
                fn set_status(&mut self, status: String) {
                    self.state.status = status;
                }
                fn pending_pastes(&mut self) -> &mut Vec<PendingTextPaste> {
                    &mut self.state.attachments.text
                }
            }
        }
    };
}
