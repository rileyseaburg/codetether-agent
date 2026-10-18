//! Native adapter for shared accounted worker completion.
mod host;
use super::session_context::Context;
use crate::{provider::CompletionResponse, session::Session};
pub(super) async fn complete(
    ctx: &Context<'_>,
    session: &mut Session,
) -> anyhow::Result<CompletionResponse> {
    codetether_a2a_worker_core::session_protocol::completion::accounted::complete(
        &mut host::Native {
            context: ctx,
            session,
        },
    )
    .await
}
