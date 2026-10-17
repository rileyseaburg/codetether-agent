//! extract recognises source extensions without slash policy.
use super::*;

#[test]
pub(super) fn extract_recognises_source_extensions_without_slash() {
    let meta = extract(&text("check lib.rs and index.tsx"));
    assert!(meta.files.iter().any(|f| f == "lib.rs"));
    assert!(meta.files.iter().any(|f| f == "index.tsx"));
}
