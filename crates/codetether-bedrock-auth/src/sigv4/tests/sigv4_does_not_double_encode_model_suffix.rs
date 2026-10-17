//! Preserved canonicalization regression.

#[test]
fn sigv4_does_not_double_encode_model_suffix() {
    let url =
        "https://bedrock-runtime.us-east-1.amazonaws.com/model/amazon.nova-lite-v1%3A0/converse";
    let canonical = crate::sigv4::canonicalize_url(url).unwrap();
    assert_eq!(
        canonical.canonical_uri,
        "/model/amazon.nova-lite-v1%3A0/converse"
    );
}
