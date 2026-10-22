//! Frontend state and callback declarations for ordered event processing.

/// Declare a frontend event pipeline with its existing stage callbacks.
///
/// State types select the application, session slot, worker bridge and event.
/// Retention/context callbacks follow the shared pipeline; lifecycle, interlude,
/// retry, tool, text, usage and error handlers preserve their existing order.
/// # Examples
/// ```text
/// codetether_session_events::declare_event_pipeline!(
///     App, SessionSlot, WorkerBridge, SessionEvent;
///     trim, context; lifecycle, interlude, retry, tools, text, usage, errors,
/// );
/// ```
#[macro_export]
macro_rules! declare_event_pipeline {
    ($app:ty, $slot:ty, $bridge:ty, $event:ty;
     $trim:path, $context:path;
     $lifecycle:path, $interlude:path, $retry:path, $tools:path, $text:path, $usage:path, $errors:path $(,)?) => {
        mod host {
            pub(super) struct Native<'a> {
                pub app: &'a mut $app,
                pub slot: &'a mut $slot,
                pub worker_bridge: &'a Option<$bridge>,
            }
            impl $crate::pipeline::View for Native<'_> {
                fn last_event_at(&mut self) -> &mut Option<::std::time::Instant> {
                    &mut self.app.state.watchdog.last_event_at
                }
                fn auto_follow(&self) -> bool {
                    self.app.state.chat_viewport.chat_auto_follow
                }
                fn scroll_to_bottom(&mut self) {
                    self.app.state.chat_viewport.scroll_to_bottom();
                }
                fn trim(&mut self) {
                    $trim(self.app);
                }
                fn context(&mut self, event: $event) {
                    $context(self.app, event);
                }
            }
            impl $crate::pipeline::Stages for Native<'_> {
                type Event = $event;
                async fn lifecycle(&mut self, event: $event) -> Option<$event> {
                    $lifecycle(self.app, self.slot, self.worker_bridge, event).await
                }
                fn interlude(&mut self, event: $event) -> Option<$event> {
                    $interlude(self.app, event)
                }
                fn retry(&mut self, event: $event) -> Option<$event> {
                    $retry(self.app, event)
                }
                async fn tools(&mut self, event: $event) -> Option<$event> {
                    $tools(self.app, self.worker_bridge, event).await
                }
                fn text(&mut self, event: $event) -> Option<$event> {
                    $text(self.app, event)
                }
                fn usage(&mut self, event: $event) -> Option<$event> {
                    $usage(self.app, event)
                }
                async fn errors(&mut self, event: $event) -> Option<$event> {
                    $errors(self.app, self.slot, self.worker_bridge, event).await
                }
            }
        }

        /// Process one runtime event through the ordered frontend pipeline.
        ///
        /// Records activity, applies auto-follow, runs event stages, trims
        /// retained state and delivers any remaining event to the context view.
        pub(crate) async fn handle_session_event(
            app: &mut $app,
            slot: &mut $slot,
            worker_bridge: &Option<$bridge>,
            evt: $event,
        ) {
            app.state.last_activity_at = ::std::time::SystemTime::now();
            $crate::pipeline::handle(
                &mut host::Native {
                    app,
                    slot,
                    worker_bridge,
                },
                evt,
            )
            .await;
        }
    };
}
