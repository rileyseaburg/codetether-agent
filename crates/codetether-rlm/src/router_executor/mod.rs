//! Router-backed compatibility executor using shared provider and REPL services.

use anyhow::Result;
use std::sync::Arc;

use crate::context_trace::{ContextTrace, ContextTraceSummary};
use crate::oracle::TraceStep;
use crate::{RlmAnalysisResult, RlmConfig};
use codetether_provider_core::Provider;

mod analysis;
mod context;
mod trace;

/// Legacy-compatible `RlmExecutor` facade over [`crate::router`].
///
/// # Examples
/// ```no_run
/// # fn example(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) {
/// let executor = codetether_rlm::router_executor::RlmExecutor::new(
///     "context".into(), provider, "model".into(),
/// );
/// assert!(executor.trace_steps().is_empty());
/// # }
/// ```
pub struct RlmExecutor {
    pub(super) context: String,
    pub(super) provider: Arc<dyn Provider>,
    pub(super) model: String,
    pub(super) max_iterations: usize,
    pub(super) trace_steps: Vec<TraceStep>,
    pub(super) context_trace: ContextTrace,
}

impl RlmExecutor {
    /// Create an executor for `context`, `provider`, and `model`.
    pub fn new(context: String, provider: Arc<dyn Provider>, model: String) -> Self {
        Self {
            context,
            provider,
            model,
            max_iterations: RlmConfig::default().max_iterations,
            trace_steps: Vec::new(),
            context_trace: ContextTrace::new(32_768),
        }
    }

    /// Set maximum router iterations.
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    /// Compatibility no-op; the engine owns temperature policy.
    pub fn with_temperature(self, _temperature: f32) -> Self {
        self
    }

    /// Compatibility no-op; progress is emitted through router events.
    pub fn with_verbose(self, _verbose: bool) -> Self {
        self
    }

    /// Trace steps captured from the latest run.
    pub fn trace_steps(&self) -> &[TraceStep] {
        &self.trace_steps
    }

    /// Context trace summary for the latest run.
    pub fn context_trace_summary(&self) -> ContextTraceSummary {
        self.context_trace.summary()
    }

    /// Execute analysis through the crate RLM router.
    pub async fn analyze(&mut self, query: &str) -> Result<RlmAnalysisResult> {
        analysis::analyze(self, query).await
    }
}
