//! Legacy executor query structured.
use super::*;
impl RlmExecutor {
    /// Handle an `rlm_llm_query` tool call from the structured path.
    ///
    /// This is the equivalent of `handle_llm_query` but takes pre-parsed
    /// parameters instead of a raw DSL line.
    pub(super) async fn handle_llm_query_direct(
        &mut self,
        query: &str,
        context_slice: Option<String>,
    ) -> Result<String> {
        self.context_trace.log_event(ContextEvent::LlmQueryResult {
            query: query.to_string(),
            response_preview: "[pending]".to_string(),
            tokens: ContextTrace::estimate_tokens(query),
        });
        let context_to_analyze = context_slice
            .clone()
            .unwrap_or_else(|| self.repl.context().to_string());

        let (answer, tokens_used) = crate::legacy_support::subquery::complete(
            self.provider.as_ref(),
            &self.model,
            &context_to_analyze,
            query,
        )
        .await?;

        self.sub_queries.push(SubQuery {
            query: query.to_string(),
            context_slice,
            response: answer.clone(),
            tokens_used: tokens_used,
        });
        self.context_trace.log_event(ContextEvent::LlmQueryResult {
            query: query.to_string(),
            response_preview: truncate_with_ellipsis(&answer, 240),
            tokens: tokens_used,
        });

        Ok(format!("llm_query result: {}", answer))
    }
}
