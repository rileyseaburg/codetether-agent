//! Validate a legacy DSL final answer against recorded pattern evidence.
use super::*;
/// Loop decision after checking a DSL execution result.
pub(super) enum DslControl {
    NoFinal,
    Rejected,
    Accepted(String),
}
impl RlmExecutor {
    pub(super) fn check_dsl_final(
        &mut self,
        code: &str,
        execution_result: &ReplResult,
        iterations: usize,
        requires_pattern_evidence: bool,
        messages: &mut Vec<Message>,
    ) -> DslControl {
        if let Some(answer) = &execution_result.final_answer {
            let stdout_has_pattern_evidence = code.contains("grep(")
                || execution_result.stdout.contains("(no matches)")
                || !Self::parse_line_numbered_output(&execution_result.stdout).is_empty();
            if requires_pattern_evidence
                && !self.has_pattern_evidence()
                && !stdout_has_pattern_evidence
            {
                self.reject_dsl_final(iterations, messages);
                return DslControl::Rejected;
            }
            self.context_trace.log_event(ContextEvent::Final {
                answer: truncate_with_ellipsis(answer, 400),
                tokens: ContextTrace::estimate_tokens(answer),
            });
            return DslControl::Accepted(answer.clone());
        }
        DslControl::NoFinal
    }
}
