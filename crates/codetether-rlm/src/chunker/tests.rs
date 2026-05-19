//! Chunker tests.

use super::RlmChunker;
use super::types::ContentType;

#[test]
fn detect_code() {
    let content = "fn main() {\n    println!(\"Hello\");\n}\n\nimpl Foo {\n    pub fn new() -> Self { Self {} }\n}";
    assert_eq!(RlmChunker::detect_content_type(content), ContentType::Code);
}

#[test]
fn detect_conversation() {
    let content = "[User]: Can you help?\n[Assistant]: Of course!\n[User]: Implement a feature.";
    assert_eq!(
        RlmChunker::detect_content_type(content),
        ContentType::Conversation
    );
}

#[test]
fn compress_within_budget() {
    let content = "line\n".repeat(1000);
    let compressed = RlmChunker::compress(&content, 100, None);
    let tokens = RlmChunker::estimate_tokens(&compressed);
    assert!(tokens <= 100 || compressed.contains("[..."));
}
