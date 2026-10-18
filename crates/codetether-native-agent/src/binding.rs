//! Concrete session field access for native agent-runtime services.
/// Implement native agent session services on an application-local marker.
///
/// The callbacks preserve the application's prior-context access and input
/// enrichment policies. The session retains its normal creation and mutation
/// methods; no separate transcript or accounting state is introduced.
/// The invocation module must be able to access metadata.directory, messages,
/// tool_uses and usage, plus the session's new/add_human_message/add_message methods.
/// # Usage
/// ```text
/// codetether_native_agent::impl_session_services!(
///     Services, Session, tool_available, prior_context_blocked, enrich_input,
/// );
/// ```
#[macro_export]
macro_rules! impl_session_services {
    ($services:ty, $session:ty, $available:path, $blocked:path, $enrich:path $(,)?) => {
        impl $crate::__private::SessionView for $services {
            type Session = $session;
            fn directory(session: &$session) -> Option<&::std::path::Path> {
                session.metadata.directory.as_deref()
            }
            fn messages(session: &$session) -> &[$crate::__private::Message] {
                &session.messages
            }
            fn tool_uses(session: &$session) -> &[$crate::__private::ToolUse] {
                &session.tool_uses
            }
            fn usage(session: &$session) -> &$crate::__private::Usage {
                &session.usage
            }
            fn available(session: &$session, name: &str) -> bool {
                $available(session, name)
            }
            fn prior_blocked(
                session: &$session,
                name: &str,
                args: &str,
            ) -> Option<$crate::__private::ToolResult> {
                $blocked(session, name, args)
            }
        }
        #[$crate::__private::async_trait]
        impl $crate::__private::SessionWrite for $services {
            async fn new_session() -> $crate::__private::Result<$session> {
                <$session>::new().await
            }
            fn add_human(session: &mut $session, message: $crate::__private::Message) {
                session.add_human_message(message);
            }
            fn add_message(session: &mut $session, message: $crate::__private::Message) {
                session.add_message(message);
            }
            fn record_tool(session: &mut $session, usage: $crate::__private::ToolUse) {
                session.tool_uses.push(usage);
            }
        }
        impl $crate::SessionServices for $services {
            fn enrich(
                input: &$crate::__private::Value,
                cwd: &::std::path::Path,
                session: &$session,
            ) -> $crate::__private::Value {
                $enrich(input, cwd, session)
            }
        }
    };
}
