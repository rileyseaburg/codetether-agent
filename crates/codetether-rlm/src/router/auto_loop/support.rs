//! Loop support helpers (progress, abort, provider selection, rewrite).

use super::super::types::{CrateAutoProcessContext, ProcessProgress};
use crate::traits::{LlmProvider, LlmResponse, ToolDefinition};
use std::sync::Arc;

pub(super) fn emit_progress(ctx: &CrateAutoProcessContext<'_>, iter: usize, max: usize) {
    if let Some(ref p) = ctx.on_progress {
        p(ProcessProgress {
            iteration: iter,
            max_iterations: max,
            status: "running".into(),
        });
    }
    if let Some(ref bus) = ctx.bus {
        let trace_id = ctx.trace_id.unwrap_or_else(uuid::Uuid::new_v4);
        bus.emit_progress(crate::RlmProgressEvent {
            trace_id,
            iteration: iter,
            max_iterations: max,
            status: "running".into(),
        });
    }
}

pub(super) fn check_abort(ctx: &CrateAutoProcessContext<'_>) -> bool {
    ctx.abort.as_ref().is_some_and(|r| *r.borrow())
}

pub(super) fn active_provider(
    ctx: &CrateAutoProcessContext<'_>,
    iter: usize,
) -> (Arc<dyn LlmProvider>, String) {
    if iter > 1 && ctx.subcall_provider.is_some() {
        (
            Arc::clone(ctx.subcall_provider.as_ref().unwrap()),
            ctx.subcall_model.as_deref().unwrap_or(&ctx.model).into(),
        )
    } else {
        (Arc::clone(&ctx.provider), ctx.model.clone())
    }
}

pub(super) async fn maybe_rewrite(
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
