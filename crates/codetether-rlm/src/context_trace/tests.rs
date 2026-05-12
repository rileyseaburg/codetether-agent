use super::trace::MAX_EVENTS;
use super::*;

fn prompt(tokens: usize) -> ContextEvent {
    ContextEvent::SystemPrompt {
        content: "test".to_string(),
        tokens,
    }
}

#[test]
fn new_trace_has_zero_tokens() {
    let trace = ContextTrace::new(1000);
    assert_eq!(trace.total_tokens(), 0);
    assert_eq!(trace.remaining_tokens(), 1000);
}

#[test]
fn log_event_adds_tokens() {
    let mut trace = ContextTrace::new(1000);
    trace.log_event(prompt(100));
    assert_eq!(trace.total_tokens(), 100);
    assert_eq!(trace.remaining_tokens(), 900);
}

#[test]
fn budget_exceeded_check() {
    let mut trace = ContextTrace::new(100);
    trace.log_event(prompt(150));
    assert!(trace.is_over_budget());
}
mod event_filter;
mod eviction;
mod summary_stats;
mod token_count;
