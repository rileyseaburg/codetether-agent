//! project bucket maps axes correctly policy.
use super::*;

#[test]
pub(super) fn project_bucket_maps_axes_correctly() {
    let meta = RelevanceMeta {
        files: vec!["src/a.rs".into()],
        tools: Vec::new(),
        error_classes: Vec::new(),
        explicit_refs: Vec::new(),
    };
    let bucket = meta.project_bucket();
    assert_eq!(bucket.tool_use, ToolUse::No);
    assert_eq!(bucket.dependency, Dependency::Chained); // single path contains '/'
    assert_eq!(bucket.difficulty, Difficulty::Easy);
}
