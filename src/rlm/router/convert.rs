//! Convert host `AutoProcessContext` to crate's `CrateAutoProcessContext`.

use super::adapter_bus::BusWrap;
use super::adapter_provider::ProviderWrap;
use super::context::AutoProcessContext;
use codetether_rlm::router::CrateAutoProcessContext;
use std::sync::Arc;

/// Convert the host context to the crate's internal form.
pub(super) fn convert_ctx(ctx: AutoProcessContext<'_>) -> CrateAutoProcessContext<'_> {
    CrateAutoProcessContext {
        tool_id: ctx.tool_id,
        tool_args: ctx.tool_args,
        session_id: ctx.session_id,
        abort: ctx.abort,
        on_progress: ctx.on_progress,
        provider: Arc::new(ProviderWrap(ctx.provider)),
        model: ctx.model,
        bus: ctx
            .bus
            .map(|b| Arc::new(BusWrap(b)) as Arc<dyn codetether_rlm::traits::RlmEventBus>),
        trace_id: ctx.trace_id,
        subcall_provider: ctx
            .subcall_provider
            .map(|p| Arc::new(ProviderWrap(p)) as Arc<dyn codetether_rlm::traits::LlmProvider>),
        subcall_model: ctx.subcall_model,
        rewriter: None,
    }
}
