use super::*;

#[test]
fn test_parse_line_numbered_output() {
    let parsed = parse_line_numbered_output("570:async fn analyze\nL1038:async fn x");
    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0], (570, "async fn analyze".to_string()));
    assert_eq!(parsed[1], (1038, "async fn x".to_string()));
}

#[test]
fn test_infer_file_from_query() {
    let file = infer_file_from_query("Find all occurrences of 'async fn' in src/rlm/repl.rs");
    assert_eq!(file.as_deref(), Some("src/rlm/repl.rs"));
}
