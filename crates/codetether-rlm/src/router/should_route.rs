//! Apply mode and output-capability gates before pressure routing.

mod pressure;
use super::types::{RoutingContext, RoutingResult};
use crate::{
    capability::{OutputCapability, output_capability},
    chunker::RlmChunker,
    config::RlmConfig,
};
use pressure::{no_route, overflow_check};

/// Decide whether tool output should be routed through RLM.
///
/// Gated on [`output_capability`]: only
/// [`OutputCapability::BulkSummarizable`] tools may be destructively
/// summarised. `ExactContent` and `Unknown` fail closed.
///
/// # Arguments
/// `output` is the tool output, `ctx` describes current context usage, and
/// `config` selects routing mode and threshold.
///
/// # Returns
/// The routing decision, reason, and estimated token count.
///
/// # Examples
/// ```
/// use codetether_rlm::{should_route, RoutingContext, RlmConfig};
/// let context = RoutingContext { tool_id: "read".into(), session_id: "s".into(),
///     call_id: None, model_context_limit: 1000, current_context_tokens: None };
/// assert!(!should_route("source", &context, &RlmConfig::default()).should_route);
/// ```
pub fn should_route(output: &str, ctx: &RoutingContext, config: &RlmConfig) -> RoutingResult {
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
        return RoutingResult {
            should_route: true,
            reason: "rlm_mode_always".into(),
            estimated_tokens: estimated,
        };
    }

    let threshold = (ctx.model_context_limit as f64 * config.threshold) as usize;
    if estimated > threshold {
        return RoutingResult {
            should_route: true,
            reason: "exceeds_threshold".into(),
            estimated_tokens: estimated,
        };
    }

    overflow_check(ctx, estimated)
}
