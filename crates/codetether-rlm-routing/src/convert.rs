//! Convert host `AutoProcessContext` to crate's `CrateAutoProcessContext`.

use super::bus::BusWrap;
use super::context::AutoProcessContext;
use codetether_rlm::provider_adapter::ProviderWrap;
use codetether_rlm::router::CrateAutoProcessContext;
use std::sync::Arc;

/// Convert the host context to the crate's internal form.
/// # Arguments
/// The application context whose owned handles are moved into adapters.
/// # Returns
/// A core context with provider and bus wrappers and no response rewriter.
/// # Examples
/// ```rust,no_run
/// # fn example<B: codetether_rlm_routing::Bus>(ctx: codetether_rlm_routing::AutoProcessContext<'_, B>) {
/// let id = ctx.tool_id;
/// let converted = codetether_rlm_routing::convert_ctx(ctx);
/// assert_eq!(converted.tool_id, id);
/// assert!(converted.rewriter.is_none());
/// # }
/// ```
pub fn convert_ctx<B: crate::Bus>(ctx: AutoProcessContext<'_, B>) -> CrateAutoProcessContext<'_> {
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
