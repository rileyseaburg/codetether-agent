use super::*;

#[test]
fn summary_statistics() {
    let mut trace = ContextTrace::new(1000);
    trace.log_event(prompt(100));
    trace.log_event(ContextEvent::GrepResult {
        pattern: "async".to_string(),
        matches: 5,
        tokens: 50,
    });
    trace.next_iteration();

    let summary = trace.summary();
    assert_eq!(summary.total_tokens, 150);
    assert_eq!(summary.iteration, 1);
    assert_eq!(summary.budget_used_percent, 15.0);
}
