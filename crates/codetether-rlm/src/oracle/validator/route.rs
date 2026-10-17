//! Select the validator for each parsed payload.

use super::{
    FinalPayload, OracleResult, TraceValidator, ValidatedTrace, validate_ast_payload,
    validate_grep_payload,
};
impl TraceValidator {
    pub(super) fn route(
        &self,
        final_payload: &FinalPayload,
        query: &str,
        source: &str,
        answer: &str,
        base_trace: impl FnOnce() -> ValidatedTrace,
    ) -> OracleResult {
        match final_payload {
            FinalPayload::Grep(_) => validate_grep_payload(
                final_payload,
                source,
                self.config.confidence_threshold,
                base_trace,
            ),
            FinalPayload::Ast(_) => validate_ast_payload(
                final_payload,
                source,
                self.config.confidence_threshold,
                base_trace,
            ),
            FinalPayload::Semantic(_) => super::fallback::semantic(base_trace),
            FinalPayload::Malformed { error, raw } => {
                self.malformed(error, raw, query, source, answer, base_trace)
            }
        }
    }
}
