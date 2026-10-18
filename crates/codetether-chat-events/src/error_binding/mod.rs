//! Frontend error presentation and failover state projection.
mod view;

/// Declare error-event handling with existing lifecycle and failover services.
///
/// The native adapter supplies app, slot and worker_bridge fields. Callbacks
/// retain inbox settlement and failover policy; this declaration projects the
/// existing model, retry, inflight prompt and status state without duplicating it.
/// # Examples
/// ```text
/// codetether_chat_events::declare_error_events!(
///     Native; App, SessionSlot, WorkerBridge, SessionEvent; settle, schedule,
/// );
/// ```
#[macro_export]
macro_rules! declare_error_events {
    ($native:ident; $(,)? $app:ty, $slot:ty, $bridge:ty, $event:ty;
     $(,)? $settle:path, $schedule:path $(,)?) => {
        mod smart_switch {
            pub(super) fn schedule(app: &mut $app, slot: &$slot, err: &str) {
                let state = &mut app.state;
                $schedule(
                    &mut state.retry_state,
                    slot.view().model.as_deref(),
                    &state.model_picker.available_models,
                    state.watchdog.inflight_prompt.as_deref(),
                    &mut state.status,
                    err,
                );
            }
        }
        $crate::__impl_error_view!($native, $settle);

        pub(super) async fn handle_event(
            app: &mut $app,
            slot: &mut $slot,
            worker_bridge: &Option<$bridge>,
            evt: $event,
        ) -> Option<$event> {
            $crate::error::handle_event(
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
