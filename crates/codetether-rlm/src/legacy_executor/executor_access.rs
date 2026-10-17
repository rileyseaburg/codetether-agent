//! Legacy executor access.
use super::*;
impl RlmExecutor {
    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.max_iterations = max;
        self
    }

    /// Set root analysis temperature for top-level RLM iterations.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.analysis_temperature = temperature.clamp(0.0, 2.0);
        self
    }

    /// Enable or disable verbose mode
    ///
    /// When verbose is true, the context summary will be displayed
    /// at the start of analysis to help users understand what's being analyzed.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Trace steps captured from the most recent analysis run.
    pub fn trace_steps(&self) -> &[TraceStep] {
        &self.trace_steps
    }

    /// Context trace summary for the most recent analysis run.
    pub fn context_trace_summary(&self) -> ContextTraceSummary {
        self.context_trace.summary()
    }
}
