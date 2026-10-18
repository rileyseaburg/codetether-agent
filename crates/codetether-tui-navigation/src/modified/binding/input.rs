//! Frontend input effects for modifier shortcuts.
/// Implementation detail of [crate::declare_frontend_modified_keys].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_modified_input {
    ($question:path, $reply:path, $transcript:path, $voice:path, $paste:path, $cancel:path) => {
        mod input {
            use super::{Action, App};
            pub(super) fn apply(app: &mut App, action: &Action) -> bool {
                match action {
                    Action::SideQuestion => {
                        $question(app);
                    }
                    Action::CopyReply => {
                        $reply(app);
                    }
                    Action::CopyTranscript => {
                        $transcript(app);
                    }
                    Action::Voice => {
                        $voice(app);
                    }
                    Action::Paste => {
                        $paste(app);
                    }
                    Action::CancelWatchdog => {
                        $cancel(&mut app.state);
                    }
                    _ => return false,
                }
                true
            }
        }
    };
}
