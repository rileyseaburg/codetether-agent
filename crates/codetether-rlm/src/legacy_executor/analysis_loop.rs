//! Drive bounded legacy analysis using explicit per-run state.
use super::*;
impl RlmExecutor {
    /// Execute RLM analysis with the given query
    ///
    /// # Arguments
    /// * `query` - Question to answer from the executor's loaded context.
    /// # Returns
    /// The final answer, sub-query records, and accumulated run statistics.
    /// # Errors
    /// Propagates provider failures, completion timeouts, and sub-query errors.
    pub async fn analyze(&mut self, query: &str) -> Result<RlmAnalysisResult> {
        let mut run = self.prepare_analysis_run(query);
        while run.iterations < self.max_iterations {
            self.execute_analysis_step(&mut run).await?;
            if run.final_answer.is_some() {
                break;
            }
        }

        self.finish_analysis(
            query,
            run.final_answer,
            run.start,
            (
                run.iterations,
                run.total_input_tokens,
                run.total_output_tokens,
            ),
        )
    }
}
