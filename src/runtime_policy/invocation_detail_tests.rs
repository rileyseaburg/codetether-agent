use serde_json::json;

#[test]
fn patch_detail_is_not_truncated() {
    let patch = format!("--- a/file\n+++ b/file\n{}", "+full line\n".repeat(100));
    assert_eq!(
        super::render("apply_patch", &json!({"patch": patch})),
        Some(patch)
    );
}
