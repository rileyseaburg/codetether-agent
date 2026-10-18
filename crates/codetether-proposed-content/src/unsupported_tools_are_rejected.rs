//! Existing proposed-content regression case.
use crate::contents;
use crate::supported;
use serde_json::json;

#[test]
fn unsupported_tools_are_rejected() {
    assert!(!supported("bash"));
    assert!(contents(std::path::Path::new("/"), "bash", &json!({})).is_err());
}
