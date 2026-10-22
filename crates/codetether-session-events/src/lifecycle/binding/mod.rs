//! Frontend lifecycle implementation declarations, split by responsibility.
mod completion;
mod processing;
mod sync;

/// Bind lifecycle transitions to an existing frontend adapter and session.
///
/// Processing, completion cleanup and session synchronization retain the
/// frontend's fields and callbacks. The native adapter is an imported type name
/// with one lifetime parameter and app, slot and worker_bridge fields.
/// # Examples
/// ```text
/// codetether_session_events::declare_event_lifecycle!(
///     Native; Session, SessionEvent, App, SessionSlot, WorkerBridge;
///     processing_started, processing_stopped,
/// );
/// ```
#[macro_export]
macro_rules! declare_event_lifecycle {
    ($native:ident; $(,)? $session:ty, $event:ty, $app:ty, $slot:ty, $bridge:ty;
     $(,)? $started:path, $stopped:path $(,)?) => {
        $crate::__impl_event_processing!($native, $started, $stopped);
        $crate::__impl_event_completion!($native);
        $crate::__impl_event_sync!($native, $session);

        pub(super) async fn handle_event(
            app: &mut $app,
            slot: &mut $slot,
            worker_bridge: &Option<$bridge>,
            evt: $event,
        ) -> Option<$event> {
            $crate::lifecycle::handle_event(
                &mut $native {
                    app,
                    slot,
                    worker_bridge,
                },
                evt,
            )
            .await
        }
    };
}
