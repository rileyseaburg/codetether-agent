//! Frontend buffer actions selected by shared text-input policy.
/// Implementation detail of [crate::declare_frontend_text_input].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_text_buffer {
    () => {
        mod buffer {
            use super::{Action, App};
            pub(super) fn apply(app: &mut App, action: &Action) -> bool {
                match action {
                    Action::Insert(c) => {
                        app.state.composer.insert_char(*c);
                    }
                    Action::DeleteBackspace => {
                        app.state.composer.delete_backspace();
                    }
                    _ => return false,
                }
                true
            }
        }
    };
}
