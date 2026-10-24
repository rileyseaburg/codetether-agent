use crate::support::{body, candidate, frame, inner, state};
use codetether_gemini_web_protocol::images::parse_response;
use serde_json::json;

#[test]
fn latest_snapshot_replaces_stale_images_and_trailers_do_not_clear_it() {
    let old = frame(inner(vec![candidate(&["stale"], "rc_old")]));
    let new = frame(inner(vec![candidate(&["a", "b", "a"], "rc_test")]));
    let bytes = body(vec![
        old,
        new,
        frame(json!([null, null, null])),
        json!(["e", 50]),
    ]);
    let result = parse_response(&bytes).unwrap();
    assert_eq!(result.urls, ["a", "b"]);
    assert_eq!(result.continuation, Some(state()));
}

#[test]
fn multiple_image_candidates_are_deduplicated_but_not_continuable() {
    let data = inner(vec![
        candidate(&["a"], "rc_a"),
        candidate(&["a", "b"], "rc_b"),
    ]);
    let result = parse_response(&body(vec![frame(data)])).unwrap();
    assert_eq!(result.urls, ["a", "b"]);
    assert!(result.continuation.is_none());
}

#[test]
fn missing_or_invalid_metadata_never_reuses_stale_state() {
    for metadata in [json!(null), json!(["bad", "r_test"])] {
        let mut data = inner(vec![candidate(&["new"], "rc_test")]);
        data[1] = metadata;
        let first = frame(inner(vec![candidate(&["old"], "rc_test")]));
        let result = parse_response(&body(vec![first, frame(data)])).unwrap();
        assert_eq!(result.urls, ["new"]);
        assert!(result.continuation.is_none());
    }
}

#[test]
fn candidate_snapshots_remove_images_without_accepting_prose_links() {
    let mut no_image = candidate(&[], "rc_test");
    no_image[1] = json!(["https://example.test/not-an-attachment"]);
    let bytes = body(vec![
        frame(inner(vec![candidate(&["old"], "rc_test")])),
        frame(inner(vec![no_image])),
    ]);
    assert!(parse_response(&bytes).is_err());
    let object = json!({"12": {"7": {"0": [[[null, null, null, [null, null, null, "image"]]]]}}});
    assert!(parse_response(&body(vec![frame(inner(vec![object]))])).is_err());
}
