#[test]
fn generated_image_succeeds_without_local_artifact() {
    let result = super::result::generated("data:image/png;base64,eA==".into(), None);
    assert!(result.success);
    assert!(result.output.contains("not saved"));
    assert!(result.metadata.get("saved_path").is_none());
    assert_eq!(
        result.metadata["image_data_url"]["data_url"],
        "data:image/png;base64,eA=="
    );
}