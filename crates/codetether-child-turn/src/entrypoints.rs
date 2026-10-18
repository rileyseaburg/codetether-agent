//! Application entry points for first turns, finalization and detached collection.

/// Declare application child-turn modules using the selected runtime host.
///
/// The session and event types retain the existing task and channel contracts.
/// The generated modules preserve the application's internal entry-point paths.
/// # Examples
/// ```text
/// codetether_child_turn::declare_turn_entrypoints!(AppHost, Session, SessionEvent);
/// ```
#[macro_export]
macro_rules! declare_turn_entrypoints {
    ($host:ty, $session:ty, $event:ty $(,)?) => {
        mod spawn_run {
            pub(super) async fn kick_off(
                agent_id: &str,
                detach: bool,
            ) -> $crate::__private::Result<$crate::__private::ToolResult> {
                $crate::kick_off::<$host>(agent_id, detach).await
            }
            pub(super) mod start {
                pub(crate) async fn begin(
                    agent_id: &str,
                ) -> $crate::__private::Result<$crate::Started<$event, $session>> {
                    $crate::begin_first::<$host>(agent_id).await
                }
            }
        }
        mod message_finalize {
            pub(super) async fn finalize(
                agent_id: String,
                response: String,
                thinking: String,
                tools: Vec<$crate::__private::Value>,
                error: Option<String>,
                updated_session: Option<$session>,
            ) -> $crate::__private::ToolResult {
                $crate::finalize::<$host>(
                    agent_id,
                    response,
                    thinking,
                    tools,
                    error,
                    updated_session,
                )
                .await
            }
        }
        mod message_detach {
            pub(super) fn dispatch(
                agent_id: String,
                guard: $crate::__private::AgentRunGuard,
                rx: $crate::__private::mpsc::Receiver<$event>,
                handle: $crate::__private::ChildTask<$session>,
                receipt: Option<String>,
            ) -> $crate::__private::ToolResult {
                $crate::dispatch::<$host>(agent_id, guard, rx, handle, receipt)
            }
        }
    };
}
