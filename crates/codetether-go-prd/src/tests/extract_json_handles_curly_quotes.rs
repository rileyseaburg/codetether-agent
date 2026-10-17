//! Existing PRD response-recovery regression test.

use crate::extract::extract_json;

#[test]
fn extract_json_handles_curly_quotes() {
    let text = "\u{201c}project\u{201d}: \u{201c}test\u{201d}";
    let full = format!("{{{text}}}");
    let result = extract_json(&full).unwrap();
    serde_json::from_str::<serde_json::Value>(&result).unwrap();
}
