//! Application response handling for the native worker session loop.
use crate::{provider::CompletionResponse, session::Session, tool::ToolRegistry};
use codetether_a2a_worker_core::session_protocol::context::ResponseContext;
pub(super) struct Responses;
impl codetether_native_worker::session_loop::Responses<Session, ToolRegistry> for Responses {
    async fn process(
        context: &ResponseContext<'_, ToolRegistry>,
        session: &mut Session,
        output: &mut String,
        response: CompletionResponse,
    ) -> anyhow::Result<bool> {
        super::session_response::process_response(context, session, output, response).await
    }
}
