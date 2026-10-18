//! Frontend source effects for continuation and session resume.
/// Implementation detail of [crate::declare_frontend_continuation].
#[doc(hidden)]
#[macro_export]
macro_rules! __frontend_continuation_source {
    ($session:path, $message:path, $prompt:path) => {
        mod source {
            use super::Native;
            use $message as Message;
            use $session as Session;
            impl $crate::resume::Source for Native<'_> {
                type Session = Session;
                fn inflight(&self) -> Option<String> {
                    self.app.state.main_inflight_prompt.clone()
                }
                fn session(&self) -> Option<&Session> {
                    self.slot.borrow()
                }
                fn durable_prompt(session: &Session) -> Option<String> {
                    $prompt(&session.id)
                }
                fn messages(session: &Session) -> &[Message] {
                    &session.messages
                }
                fn is_processing(&self) -> bool {
                    self.app.state.processing
                }
            }
        }
    };
}
