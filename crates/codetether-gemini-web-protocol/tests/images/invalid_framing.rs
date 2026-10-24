use crate::support::{body, candidate, frame, inner};
use codetether_gemini_web_protocol::images::parse_response;

#[test]
fn malformed_lengths_shapes_and_utf8_have_safe_errors() {
    let cases: &[(&[u8], &str)] = &[
        (b"hello world", "missing_anti_xssi_prefix"),
        (b")]}'\n-1\n[]\n", "invalid_batch_length"),
        (b")]}'\n1.5\n[]\n", "invalid_batch_length"),
        (b")]}'\n0\n", "batch_length_limit"),
        (b")]}'\n16777217\n", "batch_length_limit"),
        (b")]}'\n99999999999999999999999\n", "batch_length_limit"),
        (b")]}'\n100\n[]\n", "truncated_rpc_stream"),
        (b")]}'\n4\n{}\n", "invalid_batch_shape"),
        (b")]}'\n5\n[1]\n", "invalid_batch_shape"),
        (b")]}'\n4\n!!\n", "invalid_batch_json"),
        (b")]}'\n5\n[\xff]\n", "invalid_batch_json"),
    ];
    for (bytes, error) in cases {
        assert_eq!(parse_response(bytes).unwrap_err().to_string(), *error);
    }
}

#[test]
fn rejects_partial_or_miscounted_batches_even_after_a_valid_image() {
    let mut bytes = body(vec![frame(inner(vec![candidate(&["safe"], "rc_test")]))]);
    bytes.extend_from_slice(b"50\n[\"signed-secret\"]\n");
    let error = parse_response(&bytes).unwrap_err();
    assert!(!format!("{error:#}").contains("signed-secret"));
    let json = r#"[["wrb.fr",null,"雪"]]"#;
    let wrong_count = json.encode_utf16().count() + 2;
    let bytes = format!(")]}}'\n{wrong_count}\n{json}\n");
    assert!(parse_response(bytes.as_bytes()).is_err());
}
