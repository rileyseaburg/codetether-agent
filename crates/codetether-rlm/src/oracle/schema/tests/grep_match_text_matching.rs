//! Regression for grep_match_text_matching.

use super::*;

#[test]
fn grep_match_text_matching() {
    let m = GrepMatch::new(42, "async fn".to_string());
    assert!(m.text_matches("pub async fn process() -> Result<()> {"));
    assert!(!m.text_matches("fn process() -> Result<()> {"));
}
