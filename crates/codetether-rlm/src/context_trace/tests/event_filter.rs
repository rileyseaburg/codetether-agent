use super::*;

#[test]
fn event_type_filtering() {
    let mut trace = ContextTrace::new(1000);
    trace.log_event(prompt(100));
    trace.log_event(ContextEvent::GrepResult {
        pattern: "async".to_string(),
        matches: 5,
        tokens: 50,
    });
    trace.log_event(prompt(75));

    let system_events = trace.events_of_type("system_prompt");
    assert_eq!(system_events.len(), 2);

    let grep_events = trace.events_of_type("grep_result");
    assert_eq!(grep_events.len(), 1);
}
