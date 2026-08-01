use super::super::Parser;
use crate::provider::StreamChunk;

#[test]
fn reasoning_delta_is_preserved_as_thinking() {
    let mut parser = Parser::default();
    let line = concat!(
        "data: ",
        r#"{"choices":[{"delta":{"reasoning":"inspect first"}}]}"#,
        "\n\n"
    );
    assert!(matches!(&parser.push(line.as_bytes())[..],
        [StreamChunk::Thinking(text)] if text == "inspect first"));
}
