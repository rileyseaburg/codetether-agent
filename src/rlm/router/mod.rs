//! RLM Router — thin adapter over [`codetether_rlm::router`].
//!
//! Provides `RlmRouter` with the same public API as the original
//! monolith. All pure logic lives in the crate.

pub mod adapter_bus;
pub mod adapter_convert;
pub mod adapter_provider;
pub mod context;
pub mod convert;
pub mod host_impl;

pub use context::{AutoProcessContext, ProcessProgress, RoutingContext, RoutingResult};

use super::RlmConfig;
use super::RlmResult;
use super::repl::{ReplRuntime, RlmRepl};
use anyhow::Result;
use codetether_rlm::router;

/// RLM Router — delegates to [`codetether_rlm::router`].
pub struct RlmRouter;

impl RlmRouter {
    /// Check if tool output should be routed through RLM.
    pub fn should_route(output: &str, ctx: &RoutingContext, config: &RlmConfig) -> RoutingResult {
        router::should_route(output, ctx, config)
    }

    /// Smart truncate large output with RLM hint.
    pub fn smart_truncate(
        output: &str,
        tool_id: &str,
        args: &serde_json::Value,
        max: usize,
    ) -> (String, bool, usize) {
        router::smart_truncate(output, tool_id, args, max)
    }

    /// Automatically process large output through RLM.
    pub async fn auto_process(
        output: &str,
        ctx: AutoProcessContext<'_>,
        config: &RlmConfig,
    ) -> Result<RlmResult> {
        let mut repl = RlmRepl::new(output.to_string(), ReplRuntime::Rust);
        let mut host = host_impl::ReplHost(&mut repl);
        let crate_ctx = convert::convert_ctx(ctx);
        router::auto_process(output, crate_ctx, config, &mut host).await
    }
}
