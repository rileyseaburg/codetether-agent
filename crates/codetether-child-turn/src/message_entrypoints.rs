//! Typed message handling and remote fallback for application callers.

/// Declare message execution and startup using an existing child-turn host.
///
/// The remote callback receives the same context, owner and detach settings as
/// the incoming parameters. Local execution retains its guard and receipt.
/// # Examples
/// ```text
/// codetether_child_turn::declare_message_entrypoints!(
///     AppHost, Session, SessionEvent, remote_message,
/// );
/// ```
#[macro_export]
macro_rules! declare_message_entrypoints {
    ($host:ty, $session:ty, $event:ty, $remote:path $(,)?) => {
        pub(super) async fn handle_message(
            params: &$crate::__private::Params,
        ) -> $crate::__private::Result<$crate::__private::ToolResult> {
            $crate::handle_message::<$host>(params).await
        }
        pub(super) mod run {
            pub(crate) async fn execute(
                agent_id: String,
                message: String,
                images: Vec<$crate::__private::MessageImage>,
                params: &$crate::__private::Params,
                guard: $crate::__private::AgentRunGuard,
                receipt: Option<String>,
            ) -> $crate::__private::Result<$crate::__private::ToolResult> {
                $crate::run_message::<$host>(agent_id, message, images, params, guard, receipt)
                    .await
            }
        }
        pub(super) mod start {
            pub(crate) async fn begin(
                agent_id: &str,
                message: String,
                images: Vec<$crate::__private::MessageImage>,
                params: &$crate::__private::Params,
            ) -> $crate::__private::Result<$crate::Started<$event, $session>> {
                $crate::begin_message::<$host>(agent_id, message, images, params).await
            }
        }
        pub(super) mod remote_fallback {
            pub(crate) async fn execute(
                target: &str,
                message: &str,
                params: &$crate::__private::Params,
            ) -> $crate::__private::Result<$crate::__private::ToolResult> {
                $remote(
                    target,
                    message,
                    params.context_id.as_deref(),
                    params.parent_session_id.as_deref(),
                    params.detach_or_default(),
                )
                .await
            }
        }
    };
}
