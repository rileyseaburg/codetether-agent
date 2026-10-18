//! Forward legacy analysis to the shared executor.
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
        self.inner.analyze(query).await
    }
}
