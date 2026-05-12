use super::*;

#[test]
fn evict_old_events_when_full() {
    let mut trace = ContextTrace::new(10000);

    for _ in 0..(MAX_EVENTS + 100) {
        trace.log_event(prompt(1));
    }

    assert!(trace.events().len() <= MAX_EVENTS);
    assert!(trace.total_tokens() <= MAX_EVENTS);
}
