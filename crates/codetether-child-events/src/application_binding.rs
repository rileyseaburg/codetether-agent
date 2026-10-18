//! Native event projection and typed application collector entry points.

/// Declare a collector using existing session, trace and approval services.
///
/// The event type supplies the normal session variants. The observer and approval
/// callbacks retain application behavior; the shared collector owns timeout,
/// cancellation and task completion. Invoke in the application's event-loop module.
///
/// # Examples
/// ```text
/// codetether_child_events::declare_application_collector!(
///     Session, SessionEvent, ApprovalRequest, observe, approve,
/// );
/// ```
#[macro_export]
macro_rules! declare_application_collector {
    ($session:ty, $event:path, $approval:ty, $observe:path, $approve:path $(,)?) => {
        mod host {
            use $crate::{Event, Host};
            use $event as SessionEvent;

            pub struct ApplicationHost;
            impl Host for ApplicationHost {
                type SourceEvent = SessionEvent;
                type Approval = $approval;
                fn observe(agent: &str, event: &SessionEvent) {
                    $observe(agent, event);
                }
                fn project(event: SessionEvent) -> Event<Self::Approval> {
                    match event {
                        SessionEvent::TextComplete(text) => Event::TextComplete(text),
                        SessionEvent::ThinkingComplete(text) => Event::ThinkingComplete(text),
                        SessionEvent::ToolCallComplete {
                            name,
                            output,
                            success,
                            ..
                        } => Event::ToolCallComplete {
                            name,
                            output,
                            success,
                        },
                        SessionEvent::Error(error) => Event::Error(error),
                        SessionEvent::ApprovalRequest(request) => Event::ApprovalRequest(request),
                        SessionEvent::Done => Event::Done,
                        _ => Event::Ignored,
                    }
                }
                fn approve(request: &Self::Approval) {
                    $approve(request);
                }
            }
        }

        pub(super) use host::ApplicationHost;
        pub(super) type ChildTask = $crate::ChildTask<$session>;

        /// Collect child events and resolve the owned task handle.
        pub(super) async fn run(
            agent_name: &str,
            rx: &mut $crate::__private::mpsc::Receiver<$event>,
            handle: ChildTask,
        ) -> (
            String,
            String,
            Vec<$crate::__private::Value>,
            Option<String>,
            Option<$session>,
        ) {
            $crate::run::<host::ApplicationHost, $session>(agent_name, rx, handle).await
        }

        #[cfg(test)]
        mod collect {
            pub(super) async fn collect_events_with_idle_timeout(
                agent_name: &str,
                rx: &mut $crate::__private::mpsc::Receiver<$event>,
                idle_timeout: ::std::time::Duration,
            ) -> $crate::EventLoopState {
                $crate::collect_events_with_idle_timeout::<super::host::ApplicationHost>(
                    agent_name,
                    rx,
                    idle_timeout,
                )
                .await
            }
        }
    };
}
