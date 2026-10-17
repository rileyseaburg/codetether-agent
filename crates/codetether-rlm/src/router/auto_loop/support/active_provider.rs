//! Loop active provider.

use crate::{router::types::CrateAutoProcessContext, traits::LlmProvider};
use std::sync::Arc;

pub(in crate::router::auto_loop) fn active_provider(
    ctx: &CrateAutoProcessContext<'_>,
    iter: usize,
) -> (Arc<dyn LlmProvider>, String) {
    if iter > 1
        && let Some(provider) = ctx.subcall_provider.as_ref()
    {
        (
            Arc::clone(provider),
            ctx.subcall_model.as_deref().unwrap_or(&ctx.model).into(),
        )
    } else {
        (Arc::clone(&ctx.provider), ctx.model.clone())
    }
}
