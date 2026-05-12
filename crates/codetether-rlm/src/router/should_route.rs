//! Routing decision logic.

use crate::capability::{OutputCapability, output_capability};
use crate::chunker::RlmChunker;
use crate::config::RlmConfig;

use super::types::{RoutingContext, RoutingResult};

/// Decide whether tool output should be routed through RLM.
///
/// Gated on [`output_capability`]: only
/// [`OutputCapability::BulkSummarizable`] tools may be destructively
/// summarised. `ExactContent` and `Unknown` fail closed.
pub fn should_route(
    output: &str,
    ctx: &RoutingContext,
    config: &RlmConfig,
) -> RoutingResult {
    let estimated = RlmChunker::estimate_tokens(output);

    if config.mode == "off" {
        return no_route("rlm_mode_off", estimated);
    }

    let cap = output_capability(ctx.tool_id.as_str());
    if !matches!(cap, OutputCapability::BulkSummarizable) {
        let reason = match cap {
            OutputCapability::ExactContent => "tool_exact_content_no_route",
            OutputCapability::Unknown => "tool_unknown_capability_no_route",
            OutputCapability::BulkSummarizable => "tool_eligible",
        };
        return no_route(reason, estimated);
    }

    if config.mode == "always" {
        return RoutingResult { should_route: true, reason: "rlm_mode_always".into(), estimated_tokens: estimated };
    }

    let threshold = (ctx.model_context_limit as f64 * config.threshold) as usize;
    if estimated > threshold {
        return RoutingResult { should_route: true, reason: "exceeds_threshold".into(), estimated_tokens: estimated };
    }

    overflow_check(ctx, estimated)
}

fn no_route(reason: &str, tokens: usize) -> RoutingResult {
    RoutingResult { should_route: false, reason: reason.into(), estimated_tokens: tokens }
}

fn overflow_check(ctx: &RoutingContext, estimated: usize) -> RoutingResult {
    let Some(current) = ctx.current_context_tokens else {
        return no_route("within_threshold", estimated);
    };
    let projected = current + estimated;
    let limit = (ctx.model_context_limit as f64 * 0.8) as usize;
    let dominates = estimated * 2 >= projected;
    if projected > limit && dominates {
        RoutingResult { should_route: true, reason: "would_overflow".into(), estimated_tokens: estimated }
    } else {
        no_route("within_threshold", estimated)
    }
}
