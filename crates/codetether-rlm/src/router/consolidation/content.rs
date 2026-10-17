//! Chunk boundaries, routing gates, and formatting snapshots.
use crate::{ChunkOptions, RlmChunker};
#[test]
fn content_snapshots() {
    let samples = [
        "",
        "[User]: hi\n[Assistant]: hello\n[Tool read FAILED]\nerror: bad\n# Heading",
        "fn f() {}\nstruct Thing {}\n// comment\n雪だるま",
        "2026-10-17 INFO started\nTRACE event\nordinary text",
    ];
    let values:Vec<_>=samples.iter().map(|s|serde_json::json!({"kind":RlmChunker::detect_content_type(s),"chunks":RlmChunker::chunk(s,Some(ChunkOptions{max_chunk_tokens:4,preserve_recent:2}))})).collect();
    println!("CHUNKS={}", serde_json::to_string(&values).unwrap());
    super::routing_cases::capture();
    let mut formats = Vec::new();
    for tool in ["read", "bash", "grep", "unknown"] {
        formats.push(crate::smart_truncate(
            &"hello 雪 ".repeat(100),
            tool,
            &serde_json::json!({"filePath":"main.rs","pattern":"needle","include":"*.rs"}),
            20,
        ));
    }
    println!("TRUNCATION={}", serde_json::to_string(&formats).unwrap());
    let source =
        "[User]: inspect main.rs\n[Tool read]: source\nerror: first\ntrace.py\n".repeat(30);
    println!(
        "FALLBACK={}",
        serde_json::to_string(&super::super::fallback_context::session_context_fallback(
            &source, 500
        ))
        .unwrap()
    );
}
