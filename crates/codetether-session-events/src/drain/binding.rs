//! Application state adapters for bounded session-event drains.

/// Declare bounded event-drain functions using the frontend's state types.
///
/// The callback applies individual events to the app, session slot and optional
/// worker bridge. The shared drain retains batch limits and coalesces adjacent
/// cumulative text snapshots before invoking that callback.
/// # Examples
/// ```text
/// codetether_session_events::declare_event_drain!(
///     Session, SessionEvent, App, SessionSlot, WorkerBridge, handle_event,
/// );
/// ```
#[macro_export]
macro_rules! declare_event_drain {
    ($session:ty, $event:ty, $app:ty, $slot:ty, $bridge:ty, $handle:path $(,)?) => {
        mod host {
            pub(super) struct Native<'a> {
                pub app: &'a mut $app,
                pub slot: &'a mut $slot,
                pub worker_bridge: &'a Option<$bridge>,
            }
            impl $crate::drain::Handler<$session> for Native<'_> {
                async fn handle(&mut self, event: $event) {
                    $handle(self.app, self.slot, self.worker_bridge, event).await;
                }
            }
        }

        /// Apply a small bounded batch of queued events during a normal UI pass.
        ///
        /// # Arguments
        /// Receives frontend state, the active session slot, an optional worker
        /// bridge and the queued event receiver.
        /// # Returns
        /// Unit after applying up to the shared batch limit; pending later events
        /// stay queued for subsequent passes.
        pub(crate) async fn drain_batch(
            app: &mut $app,
            slot: &mut $slot,
            worker_bridge: &Option<$bridge>,
            event_rx: &mut $crate::__private::mpsc::Receiver<$event>,
        ) {
            $crate::drain::batch(
                &mut host::Native {
                    app,
                    slot,
                    worker_bridge,
                },
                event_rx,
            )
            .await;
        }

        /// Flush queued events before processing completion or ownership changes.
        ///
        /// # Arguments
        /// Receives the same frontend, slot, bridge and receiver as the normal
        /// batch drain.
        /// # Returns
        /// Unit after applying the shared notice-drain limit and handling rules.
        pub(crate) async fn drain_before_notice(
            app: &mut $app,
            slot: &mut $slot,
            worker_bridge: &Option<$bridge>,
            event_rx: &mut $crate::__private::mpsc::Receiver<$event>,
        ) {
            $crate::drain::before_notice(
                &mut host::Native {
                    app,
                    slot,
                    worker_bridge,
                },
                event_rx,
            )
            .await;
        }
    };
}
