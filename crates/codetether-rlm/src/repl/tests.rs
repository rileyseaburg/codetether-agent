//! Existing context and DSL tests.
use super::*;

#[test]
fn test_repl_head_tail() {
    let context = (1..=100)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let repl = RlmRepl::new(context, ReplRuntime::Rust);

    let head = repl.head(5);
    assert_eq!(head.len(), 5);
    assert_eq!(head[0], "line 1");

    let tail = repl.tail(5);
    assert_eq!(tail.len(), 5);
    assert_eq!(tail[4], "line 100");
}

#[test]
fn test_repl_grep() {
    let context = "error: something failed\ninfo: all good\nerror: another failure".to_string();
    let repl = RlmRepl::new(context, ReplRuntime::Rust);

    let matches = repl.grep("error");
    assert_eq!(matches.len(), 2);
}

#[test]
fn test_repl_execute_final() {
    let context = "test content".to_string();
    let mut repl = RlmRepl::new(context, ReplRuntime::Rust);

    let result = repl.execute(r#"FINAL("This is the answer")"#);
    assert_eq!(result.final_answer, Some("This is the answer".to_string()));
}

#[test]
fn test_repl_chunks() {
    let context = (1..=100)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let repl = RlmRepl::new(context, ReplRuntime::Rust);

    let chunks = repl.chunks(5);
    assert_eq!(chunks.len(), 5);
}
