//! Frontend filters actions selected by shared text-input policy.
/// Implementation detail of [crate::declare_frontend_text_input].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_text_filters {
    ($push:path, $backspace:path) => {
        mod filters {
            use super::{Action, App};
            pub(super) fn apply(app: &mut App, action: &Action) -> bool {
                match action {
                    Action::BusPush(c) => {
                        app.state.bus_log.push_filter_char(*c);
                    }
                    Action::BusPop => {
                        app.state.bus_log.pop_filter_char();
                    }
                    Action::ModelPush(c) => {
                        app.state.model_picker.model_filter_push(*c);
                    }
                    Action::ModelBackspace => {
                        app.state.model_picker.model_filter_backspace();
                    }
                    Action::FilePush(c) => {
                        $push(&mut app.state.file_picker, *c);
                    }
                    Action::FileBackspace => {
                        $backspace(&mut app.state.file_picker);
                    }
                    _ => return false,
                }
                true
            }
        }
    };
}
