use serde_json::json;

#[test]
fn metadata_lookup_reads_nested_forage_keys() {
    let metadata = json!({"forage": {"execute": true, "top": 5}})
        .as_object()
        .cloned()
        .unwrap();
    assert_eq!(super::metadata_bool(&metadata, &["execute"]), Some(true));
    assert_eq!(super::metadata_usize(&metadata, &["top"]), Some(5));
}
