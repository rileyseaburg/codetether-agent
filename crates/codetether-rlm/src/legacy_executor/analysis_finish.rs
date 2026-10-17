//! Normalize the answer and assemble the legacy run result.
use super::*;
impl RlmExecutor {
    pub(super) fn finish_analysis(
        &mut self,
        query: &str,
        final_answer: Option<String>,
        start: std::time::Instant,
        counts: (usize, usize, usize),
    ) -> Result<RlmAnalysisResult> {
        let (iterations, total_input_tokens, total_output_tokens) = counts;
        let elapsed = start.elapsed();

        let raw_final_text = final_answer.unwrap_or_else(|| "Analysis incomplete".to_string());
        let final_text = self.ensure_structured_final_payload(query, raw_final_text);
        if !matches!(
            self.context_trace.events().back(),
            Some(ContextEvent::Final { .. })
        ) {
            self.context_trace.log_event(ContextEvent::Final {
                answer: truncate_with_ellipsis(&final_text, 400),
                tokens: ContextTrace::estimate_tokens(&final_text),
            });
        }

        Ok(RlmAnalysisResult {
            answer: final_text,
            iterations,
            sub_queries: self.sub_queries.clone(),
            stats: crate::RlmStats {
                input_tokens: total_input_tokens,
                output_tokens: total_output_tokens,
                iterations,
                subcalls: self.sub_queries.len(),
                elapsed_ms: elapsed.as_millis() as u64,
                compression_ratio: 1.0,
            },
        })
    }
}
