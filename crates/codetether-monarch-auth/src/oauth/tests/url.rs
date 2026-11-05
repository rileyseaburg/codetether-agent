//! The consent URL must bind the requested scope, resource, and PKCE method.
use crate::{discovery::RESOURCE, oauth::url::authorization_url};

#[test]
fn authorization_is_read_only_and_resource_bound() {
    let url = authorization_url(
        "client",
        "http://127.0.0.1:42/callback",
        "mcp:read",
        "state",
        "verifier",
    )
    .unwrap();
    let pairs: std::collections::HashMap<_, _> = url.query_pairs().collect();
    assert_eq!(pairs["scope"], "mcp:read");
    assert_eq!(pairs["resource"], RESOURCE);
    assert_eq!(pairs["code_challenge_method"], "S256");
    assert!(!url.as_str().contains("verifier"));
}
