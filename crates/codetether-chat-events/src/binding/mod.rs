//! Application chat-state projection and typed event dispatch.
mod stream;
mod usage;

/// Declare stream and usage state bindings with typed text/retry/usage handlers.
///
/// The frontend retains its App and event types and supplies its streaming size
/// limit. Shared chat-event functions continue to own event classification and
/// presentation policy.
/// # Examples
/// ```text
/// codetether_chat_events::declare_chat_state!(App, SessionEvent, MAX_STREAMING_BYTES);
/// ```
#[macro_export]
macro_rules! declare_chat_state {
    ($app:ty, $event:ty, $limit:path $(,)?) => {
        mod chat_host {
            pub(super) struct Native<'a>(pub &'a mut $app);
            $crate::__impl_chat_stream!(Native, $limit);
            $crate::__impl_chat_usage!(Native);
        }
        mod text_dispatch {
            pub(super) fn handle_event(app: &mut $app, evt: $event) -> Option<$event> {
                $crate::handle_text_event(&mut super::chat_host::Native(app), evt)
            }
        }
        mod retry {
            pub(super) fn handle_event(app: &mut $app, evt: $event) -> Option<$event> {
                $crate::retry::handle_event(&mut super::chat_host::Native(app), evt)
            }
        }
        mod usage {
            pub use $crate::attach_usage_to_last_completion_message;
            pub(super) fn handle_event(app: &mut $app, evt: $event) -> Option<$event> {
                $crate::usage::handle_event(&mut super::chat_host::Native(app), evt)
            }
        }
    };
}
