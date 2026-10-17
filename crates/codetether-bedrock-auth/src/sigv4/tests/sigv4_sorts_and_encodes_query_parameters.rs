//! Preserved canonicalization regression.

#[test]
fn sigv4_sorts_and_encodes_query_parameters() {
    let url = "https://bedrock.us-east-1.amazonaws.com/inference-profiles?typeEquals=SYSTEM_DEFINED&maxResults=200";
    let canonical = crate::sigv4::canonicalize_url(url).unwrap();
    assert_eq!(
        canonical.canonical_querystring,
        "maxResults=200&typeEquals=SYSTEM_DEFINED"
    );
}
