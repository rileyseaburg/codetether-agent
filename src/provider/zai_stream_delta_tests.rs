use super::push_reasoning;
use crate::provider::StreamChunk;

#[test]
fn reasoning_becomes_thinking_not_answer_text() {
    let mut chunks = Vec::new();
    let mut buffered = String::new();

    push_reasoning(&mut chunks, &mut buffered, "The user wants me to run glob.");

    assert_eq!(chunks.len(), 1);
    assert!(matches!(&chunks[0], StreamChunk::Thinking(text)
        if text.contains("wants me to run glob")));
    assert!(
        !chunks
            .iter()
            .any(|chunk| matches!(chunk, StreamChunk::Text(_)))
    );
}

#[test]
fn pending_answer_text_is_flushed_before_thinking() {
    let mut chunks = Vec::new();
    let mut buffered = String::from("Here is the result.");

    push_reasoning(&mut chunks, &mut buffered, "now verifying");

    assert!(buffered.is_empty());
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == "Here is the result."));
    assert!(matches!(&chunks[1], StreamChunk::Thinking(text) if text == "now verifying"));
}

#[test]
fn repeated_reasoning_deltas_stay_separate() {
    let mut chunks = Vec::new();
    let mut buffered = String::new();

    push_reasoning(&mut chunks, &mut buffered, "step one");
    push_reasoning(&mut chunks, &mut buffered, "step two");

    let thinking: Vec<_> = chunks
        .iter()
        .filter_map(|chunk| match chunk {
            StreamChunk::Thinking(text) => Some(text.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(thinking, vec!["step one", "step two"]);
}
