//! Loop maybe rewrite.

use crate::{
    router::types::CrateAutoProcessContext,
    traits::{LlmResponse, ToolDefinition},
};

pub(in crate::router::auto_loop) async fn maybe_rewrite(
    ctx: &CrateAutoProcessContext<'_>,
    resp: LlmResponse,
    tools: &[ToolDefinition],
) -> LlmResponse {
    if let Some(ref rw) = ctx.rewriter {
        rw.maybe_reformat(resp.clone(), tools, true)
            .await
            .unwrap_or(resp)
    } else {
        resp
    }
}
