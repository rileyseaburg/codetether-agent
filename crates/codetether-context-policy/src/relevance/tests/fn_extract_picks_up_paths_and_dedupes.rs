//! extract picks up paths and dedupes policy.
use super::*;

#[test]
pub(super) fn extract_picks_up_paths_and_dedupes() {
    let meta = extract(&text(
        "Edited src/lib.rs and src/lib.rs again, plus tests/a.rs",
    ));
    assert_eq!(meta.files.len(), 2);
    assert!(meta.files.contains(&"src/lib.rs".to_string()));
    assert!(meta.files.contains(&"tests/a.rs".to_string()));
}
