//! Context trace construction for engine runs.

use crate::context_trace::{ContextEvent, ContextTrace};

use super::evidence::Evidence;

/// Build an auditable trace for selected evidence and final answer.
pub fn build(
    answer: &str,
    budget: usize,
    answer_tokens: usize,
    evidence: Option<&Evidence>,
) -> ContextTrace {
    let mut trace = ContextTrace::new(budget);
    if let Some(evidence) = evidence {
        let preview = preview(evidence);
        trace.log_event(ContextEvent::ToolResult {
            tool_call_id: "context_index".into(),
            result_preview: preview.clone(),
            tokens: ContextTrace::estimate_tokens(&preview),
        });
    }
    trace.log_event(ContextEvent::Final {
        answer: answer.chars().take(400).collect(),
        tokens: answer_tokens,
    });
    trace
}

fn preview(evidence: &Evidence) -> String {
    evidence
        .records
        .iter()
        .take(6)
        .map(|r| {
            format!(
                "{}:{}-{}:{:?}:{:.1}:{}",
                r.source, r.span.0, r.span.1, r.kind, r.score, r.reason
            )
        })
        .collect::<Vec<_>>()
        .join(" | ")
}
