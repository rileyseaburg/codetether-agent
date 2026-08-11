use super::TreeSitterOracle;

#[test]
fn rust_highlight_captures_preserve_byte_ranges() {
    let source = "fn main() { let answer = 42; }";
    let mut oracle = TreeSitterOracle::new(source.to_string());
    let captures = oracle.rust_highlight_captures().unwrap();

    assert!(!captures.is_empty());
    assert!(captures.iter().all(|capture| {
        capture.start_byte < capture.end_byte
            && capture.end_byte <= source.len()
            && !capture.name.is_empty()
    }));
    assert!(captures.iter().any(|capture| {
        capture.name == "keyword" && source.get(capture.start_byte..capture.end_byte) == Some("fn")
    }));
}
