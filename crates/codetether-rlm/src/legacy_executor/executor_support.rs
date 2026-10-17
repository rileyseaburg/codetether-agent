//! Legacy executor support.
use super::*;
impl RlmExecutor {
    pub(super) fn ensure_structured_final_payload(
        &mut self,
        query: &str,
        raw_final_text: String,
    ) -> String {
        crate::legacy_support::final_payload::ensure_structured_final_payload(
            query,
            raw_final_text,
            &mut self.trace_steps,
            &mut self.context_trace,
        )
    }

    pub(super) fn has_pattern_evidence(&self) -> bool {
        crate::legacy_support::has_pattern_evidence(&self.trace_steps)
    }

    pub(super) fn parse_line_numbered_output(output: &str) -> Vec<(usize, String)> {
        crate::legacy_support::parse_line_numbered_output(output)
    }

    pub(super) fn trace_output_for_storage(output: &str) -> String {
        crate::legacy_support::trace_output_for_storage(output)
    }

    /// Extract code from LLM response
    pub(super) fn extract_code(&self, text: &str) -> String {
        crate::legacy_support::extract_code(text)
    }

    /// Parse llm_query("question", context?) call
    pub(super) fn parse_llm_query(&mut self, line: &str) -> (String, Option<String>) {
        crate::legacy_support::parse_llm_query(line, &mut self.repl)
    }
}
