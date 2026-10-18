//! Borrow and mutate session state through the application's existing methods.
use super::Host;
use codetether_agent_core::ToolUse;
use codetether_agent_runtime::{SessionView, SessionWrite};
use codetether_provider_core::{Message, Usage};
use codetether_tool_core::ToolResult;
use std::path::Path;
impl<S: SessionView, T: Send + Sync + 'static> SessionView for Host<S, T> {
    type Session = S::Session;
    fn directory(session: &Self::Session) -> Option<&Path> {
        S::directory(session)
    }
    fn messages(session: &Self::Session) -> &[Message] {
        S::messages(session)
    }
    fn tool_uses(session: &Self::Session) -> &[ToolUse] {
        S::tool_uses(session)
    }
    fn usage(session: &Self::Session) -> &Usage {
        S::usage(session)
    }
    fn available(session: &Self::Session, name: &str) -> bool {
        S::available(session, name)
    }
    fn prior_blocked(session: &Self::Session, name: &str, args: &str) -> Option<ToolResult> {
        S::prior_blocked(session, name, args)
    }
}
#[async_trait::async_trait]
impl<S: SessionWrite, T: Send + Sync + 'static> SessionWrite for Host<S, T> {
    async fn new_session() -> anyhow::Result<Self::Session> {
        S::new_session().await
    }
    fn add_human(session: &mut Self::Session, message: Message) {
        S::add_human(session, message);
    }
    fn add_message(session: &mut Self::Session, message: Message) {
        S::add_message(session, message);
    }
    fn record_tool(session: &mut Self::Session, usage: ToolUse) {
        S::record_tool(session, usage);
    }
}
