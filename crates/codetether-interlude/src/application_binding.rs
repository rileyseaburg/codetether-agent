//! Frontend interlude and redraw state binding.

/// Declare interlude event handling for existing frontend and event types.
///
/// Delegates to the shared interactive handler with the current interlude and
/// redraw flag. Unrelated events retain the handler's normal fall-through.
/// # Examples
/// ```text
/// codetether_interlude::declare_interlude_events!(App, SessionEvent);
/// ```
#[macro_export]
macro_rules! declare_interlude_events {
    ($app:ty, $event:ty $(,)?) => {
        pub(super) fn handle_event(app: &mut $app, evt: $event) -> Option<$event> {
            $crate::interactive::handle_event(
                &mut app.state.interlude,
                &mut app.state.needs_redraw,
                evt,
            )
        }
    };
}
