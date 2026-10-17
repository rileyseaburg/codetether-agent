//! Initialize run state and reset the legacy executor trace.
use super::*;
impl RlmExecutor {
    pub(super) fn prepare_analysis_run(&mut self, query: &str) -> AnalysisRun {
        let start = std::time::Instant::now();
        let iterations = 0;
        let total_input_tokens = 0;
        let total_output_tokens = 0;
        self.sub_queries.clear();
        self.trace_steps.clear();
        self.context_trace = ContextTrace::new(self.context_budget_tokens);

        // Prepare RLM tool definitions for structured dispatch
        let tools = rlm_tool_definitions();

        // Build and optionally display context summary
        self.show_analysis_summary();
        let messages = self.prepare_analysis_messages(query);
        let llm_timeout_secs = std::env::var("CODETETHER_RLM_LLM_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse::<u64>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(60);
        let requires_pattern_evidence =
            GrepOracle::classify_query(query) == QueryType::PatternMatch;

        let final_answer = None;

        AnalysisRun {
            start,
            iterations,
            total_input_tokens,
            total_output_tokens,
            tools,
            messages,
            llm_timeout_secs,
            requires_pattern_evidence,
            final_answer,
        }
    }
}
