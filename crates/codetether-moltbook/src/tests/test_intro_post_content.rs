//! Existing test_intro_post_content test.
use super::*;

#[test]
fn test_intro_post_content() {
    let (title, content) = intro_post("MyAgent");
    assert!(title.contains("MyAgent"));
    assert!(content.contains("CodeTether"));
    assert!(content.contains("HashiCorp Vault"));
    assert!(content.contains("OPA"));
}
