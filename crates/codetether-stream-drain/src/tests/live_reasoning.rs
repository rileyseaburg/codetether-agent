use super::SessionEvent;

#[test]
fn live_reasoning_arrives_before_completion_and_recovers_dropped_updates() {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<SessionEvent>(1);
    let mut thinking = String::new();
    crate::text_acc::on_thinking(&mut thinking, "first", Some(&tx)).unwrap();
    crate::text_acc::on_thinking(&mut thinking, " second", Some(&tx)).unwrap();
    assert!(matches!(rx.try_recv(), Ok(SessionEvent::ThinkingChunk(text)) if text == "first"));
    crate::text_acc::on_thinking(&mut thinking, " third", Some(&tx)).unwrap();
    assert!(
        matches!(rx.try_recv(), Ok(SessionEvent::ThinkingChunk(text)) if text == "first second third")
    );
}
