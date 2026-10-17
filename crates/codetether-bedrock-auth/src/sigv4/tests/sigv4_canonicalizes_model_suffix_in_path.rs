//! Preserved canonicalization regression.

#[test]
fn sigv4_canonicalizes_model_suffix_in_path() {
    let url = "https://bedrock-runtime.us-east-1.amazonaws.com/model/amazon.nova-lite-v1:0/converse-stream";
    let canonical = crate::sigv4::canonicalize_url(url).unwrap();
    assert_eq!(
        canonical.canonical_uri,
        "/model/amazon.nova-lite-v1%3A0/converse-stream"
    );
}
