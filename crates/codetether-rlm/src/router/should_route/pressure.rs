//! Routing result construction and projected context pressure.

use crate::router::types::{RoutingContext, RoutingResult};

pub(super) fn no_route(reason: &str, tokens: usize) -> RoutingResult {
    RoutingResult {
        should_route: false,
        reason: reason.into(),
        estimated_tokens: tokens,
    }
}

pub(super) fn overflow_check(ctx: &RoutingContext, estimated: usize) -> RoutingResult {
    let Some(current) = ctx.current_context_tokens else {
        return no_route("within_threshold", estimated);
    };
    let projected = current + estimated;
    let limit = (ctx.model_context_limit as f64 * 0.8) as usize;
    let dominates = estimated * 2 >= projected;
    if projected > limit && dominates {
        RoutingResult {
            should_route: true,
            reason: "would_overflow".into(),
            estimated_tokens: estimated,
        }
    } else {
        no_route("within_threshold", estimated)
    }
}
