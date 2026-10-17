//! Perform one model turn and dispatch its structured or DSL response.
use super::*;
impl RlmExecutor {
    pub(super) async fn execute_analysis_step(&mut self, run: &mut AnalysisRun) -> Result<()> {
        run.iterations += 1;
        tracing::info!("RLM iteration {}", run.iterations);

        // Get LLM response with code to execute (with timeout)
        let response = self
            .request_analysis(&run.messages, &run.tools, run.llm_timeout_secs)
            .await?;
        // Optionally run FunctionGemma to convert text-only responses into
        // structured tool calls.

        let response = self.rewrite_analysis(response, &run.tools).await;
        run.total_input_tokens += response.usage.prompt_tokens;
        run.total_output_tokens += response.usage.completion_tokens;
        self.trace_analysis_response(&response);
        // ── Structured tool-call path ────────────────────────────────
        // If the response (or FunctionGemma rewrite) contains ToolCall
        // entries, dispatch them directly against the REPL.

        if self.execute_structured_step(&response, run).await? {
            return Ok(());
        }
        self.execute_analysis_dsl(&response, run).await
    }
}
