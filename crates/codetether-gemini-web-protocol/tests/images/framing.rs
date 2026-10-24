use crate::support::{batch, body, candidate, frame, inner};
use codetether_gemini_web_protocol::images::parse_response;
use serde_json::json;

#[test]
fn unicode_is_framed_by_utf8_bytes_not_characters() {
    let url = "https://images.test/雪🛶?signed=fixture";
    let mut data = inner(vec![candidate(&[url], "rc_test")]);
    data[4][0][1][0] = json!("Crème, 雪, 🛶, e\u{301}\nquoted: \"");
    let result = parse_response(&body(vec![frame(data)])).unwrap();
    assert_eq!(result.urls, vec![url]);
}

#[test]
fn accepts_multiple_batches_crlf_and_opaque_trailers() {
    let first = frame(inner(vec![candidate(&["first"], "rc_test")]));
    let last = frame(inner(vec![candidate(&["last"], "rc_test")]));
    let mut bytes = b")]}'\r\n\r\n".to_vec();
    bytes.extend(batch(&[first]));
    bytes.extend_from_slice(b"\t \r\n");
    bytes.extend(batch(&[last, json!(["e", 999]), json!(["di", 1])]));
    bytes.extend_from_slice(b" \r\n");
    assert_eq!(parse_response(&bytes).unwrap().urls, vec!["last"]);
}

#[test]
fn all_truncations_of_a_final_json_batch_are_rejected() {
    let bytes = body(vec![frame(inner(vec![candidate(&["image"], "rc_test")]))]);
    for end in 0..bytes.len() {
        assert!(parse_response(&bytes[..end]).is_err(), "end={end}");
    }
    assert!(parse_response(&bytes).is_ok());
    let mut bytes = bytes;
    bytes.extend_from_slice(b"123");
    assert!(parse_response(&bytes).is_err());
}
