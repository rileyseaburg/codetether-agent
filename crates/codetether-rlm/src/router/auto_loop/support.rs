//! Loop support helpers (progress, abort, provider selection, rewrite).

use std::sync::Arc;
use crate::traits::{LlmProvider, LlmResponse, ToolDefinition};
use super::super::types::{CrateAutoProcessContext, ProcessProgress};

pub(super) fn emit_progress(ctx: &CrateAutoProcessContext<'_>, iter: usize, max: usize) {
    if let Some(ref p) = ctx.on_progress {
        p(ProcessProgress { iteration: iter, max_iterations: max, status: "running".into() });
    }
    if let Some(ref bus) = ctx.bus {
        bus.emit_progress(crate::RlmProgressEvent {
            trace_id: uuid::Uuid::nil(), iteration: iter,
            max_iterations: max, status: "running".into(),
        });
    }
}

pub(super) fn check_abort(ctx: &CrateAutoProcessContext<'_>) -> bool {
    ctx.abort.as_ref().is_some_and(|r| *r.borrow())
}

pub(super) fn active_provider(ctx: &CrateAutoProcessContext<'_>, iter: usize) -> (Arc<dyn LlmProvider>, String) {
    if iter > 1 && ctx.subcall_provider.is_some() {
        (Arc::clone(ctx.subcall_provider.as_ref().unwrap()), ctx.subcall_model.as_deref().unwrap_or(&ctx.model).into())
    } else {
        (Arc::clone(&ctx.provider), ctx.model.clone())
    }
}

pub(super) async fn maybe_rewrite(ctx: &CrateAutoProcessContext<'_>, resp: LlmResponse, tools: &[ToolDefinition]) -> LlmResponse {
    if let Some(ref rw) = ctx.rewriter { rw.maybe_reformat(resp, tools, true).await.unwrap_or(resp) } else { resp }
}
