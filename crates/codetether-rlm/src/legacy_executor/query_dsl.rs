//! Legacy executor query dsl.
use super::*;
impl RlmExecutor {
    /// Handle llm_query() calls
    pub(super) async fn handle_llm_query(&mut self, line: &str) -> Result<String> {
        // Extract query and optional context slice
        let (query, context_slice) = self.parse_llm_query(line);
        self.context_trace.log_event(ContextEvent::LlmQueryResult {
            query: query.clone(),
            response_preview: "[pending]".to_string(),
            tokens: ContextTrace::estimate_tokens(&query),
        });

        // Get the context to send
        let context_to_analyze = context_slice
            .clone()
            .unwrap_or_else(|| self.repl.context().to_string());

        // Truncate context for sub-query to avoid overwhelming the LLM

        let (answer, tokens_used) = crate::legacy_support::subquery::complete(
            self.provider.as_ref(),
            &self.model,
            &context_to_analyze,
            &query,
        )
        .await?;

        // Record the sub-query
        self.sub_queries.push(SubQuery {
            query: query.clone(),
            context_slice,
            response: answer.clone(),
            tokens_used: tokens_used,
        });
        self.trace_steps.push(TraceStep {
            iteration: self.sub_queries.len(),
            action: format!("llm_query({})", truncate_with_ellipsis(&query, 120)),
            output: truncate_with_ellipsis(&answer, 240),
        });
        self.context_trace.log_event(ContextEvent::LlmQueryResult {
            query,
            response_preview: truncate_with_ellipsis(&answer, 240),
            tokens: tokens_used,
        });

        Ok(format!("llm_query result: {}", answer))
    }
}
