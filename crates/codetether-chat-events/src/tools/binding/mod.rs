//! Frontend tool presentation and timing declarations.
mod timing;
mod view;

/// Declare tool-event presentation using the frontend's existing state.
///
/// Callbacks retain approval delivery, current trust guidance and worker-start
/// notification. Shared tool-event logic owns event classification and messages.
/// # Examples
/// ```text
/// codetether_chat_events::declare_tool_events!(
///     App, SessionEvent, WorkerBridge; approval, guidance, processing_started,
/// );
/// ```
#[macro_export]
macro_rules! declare_tool_events {
    ($app:ty, $event:ty, $bridge:ty; $(,)? $approval:path, $guidance:path, $started:path $(,)?) => {
        pub(super) struct Native<'a> {
            pub app: &'a mut $app,
            pub worker_bridge: &'a Option<$bridge>,
        }
        $crate::__impl_tool_event_view!(Native, $approval, $guidance);
        $crate::__impl_tool_event_timing!(Native, $started);

        pub(super) async fn handle_event(
            app: &mut $app,
            worker_bridge: &Option<$bridge>,
            evt: $event,
        ) -> Option<$event> {
            $crate::tools::handle_event(&mut Native { app, worker_bridge }, evt).await
        }
    };
}
