//! Display context information before the legacy analysis run.
use super::*;
impl RlmExecutor {
    pub(super) fn show_analysis_summary(&self) {
        let context_summary = crate::legacy_support::prompts::context_summary(&self.repl);

        // Display context summary at the start in verbose mode
        if self.verbose {
            tracing::info!("RLM Context Summary:\n{}", context_summary);
            println!(
                "[RLM] Context loaded: {} chars, {} lines",
                self.repl.context().len(),
                self.repl.lines().len()
            );
        }
    }
}
