//! Compatibility builder methods and trace accessors.
use super::*;
impl RlmExecutor {
    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max: usize) -> Self {
        self.inner = self.inner.with_max_iterations(max);
        self
    }

    /// Set root analysis temperature for top-level RLM iterations.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.inner = self.inner.with_temperature(temperature);
        self
    }

    /// Enable or disable verbose mode
    ///
    /// When verbose is true, the context summary will be displayed
    /// at the start of analysis to help users understand what's being analyzed.
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.inner = self.inner.with_verbose(verbose);
        self
    }

    /// Trace steps captured from the most recent analysis run.
    pub fn trace_steps(&self) -> &[TraceStep] {
        self.inner.trace_steps()
    }

    /// Context trace summary for the most recent analysis run.
    pub fn context_trace_summary(&self) -> ContextTraceSummary {
        self.inner.context_trace_summary()
    }
}
