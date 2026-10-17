//! A provider cannot silently change the index's embedding dimensions.

use super::*;
#[tokio::test]
async fn remote_dimension_mismatch_is_rejected() {
    let (dir, resolver, options) = fixture(0, 32);
    let error = build_index_with_resolver(dir.path(), &options, &resolver)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("expected 64, got 32"));
}
