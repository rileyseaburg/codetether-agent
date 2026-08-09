use serde_json::json;

#[test]
fn revised_arguments_replace_patch_and_preserve_other_fields() {
    let original = json!({"patch": "old", "dry_run": false, "approval_id": "old-id"});
    let revision = json!({"patch": "edited"});

    let args = super::args::revised(original, revision, "edited-id");

    assert_eq!(args["patch"], "edited");
    assert_eq!(args["dry_run"], false);
    assert_eq!(args["approval_id"], "edited-id");
}
