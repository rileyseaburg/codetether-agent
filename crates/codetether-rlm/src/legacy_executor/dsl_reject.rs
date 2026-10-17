//! Record a DSL final answer rejected for missing pattern evidence.
use super::*;

impl RlmExecutor {
    pub(super) fn reject_dsl_final(&mut self, iterations: usize, messages: &mut Vec<Message>) {
        let rejection = "FINAL rejected: pattern query requires grep evidence. Run grep/head/tail before FINAL and include exact lines.";
        self.trace_steps.push(TraceStep {
            iteration: iterations,
            action: "reject_final(no_grep_evidence)".to_string(),
            output: rejection.to_string(),
        });
        self.context_trace.log_event(ContextEvent::ExecutionOutput {
            output: rejection.to_string(),
            tokens: ContextTrace::estimate_tokens(rejection),
        });
        messages.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: rejection.to_string(),
            }],
        });
    }
}
