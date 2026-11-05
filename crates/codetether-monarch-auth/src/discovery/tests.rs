//! Reject untrusted discovery endpoints and missing PKCE support.
use super::{AUTHORIZE, ISSUER, REGISTER, RESOURCE, REVOKE, TOKEN};
use super::{
    models::{ResourceMetadata, ServerMetadata},
    validate::validate,
};

#[test]
fn discovery_rejects_untrusted_endpoints_and_missing_pkce() {
    let resource = ResourceMetadata {
        resource: RESOURCE.into(),
        authorization_servers: vec![ISSUER.into()],
        scopes_supported: vec!["mcp:read".into(), "mcp:write".into()],
    };
    let mut server = ServerMetadata {
        issuer: ISSUER.into(),
        authorization_endpoint: AUTHORIZE.into(),
        token_endpoint: TOKEN.into(),
        registration_endpoint: REGISTER.into(),
        revocation_endpoint: REVOKE.into(),
        code_challenge_methods_supported: vec!["S256".into()],
        token_endpoint_auth_methods_supported: vec!["none".into()],
        response_types_supported: vec!["code".into()],
        grant_types_supported: vec!["authorization_code".into(), "refresh_token".into()],
    };
    assert!(validate(&resource, &server).is_ok());
    for endpoint in [
        "http://api.monarch.com/oauth/token/",
        "https://api.monarch.com.evil.test/oauth/token/",
        "https://api.monarch.com/oauth/token/?next=evil",
        "https://user@api.monarch.com/oauth/token/",
    ] {
        server.token_endpoint = endpoint.into();
        assert!(validate(&resource, &server).is_err());
    }
    server.token_endpoint = TOKEN.into();
    server.code_challenge_methods_supported = vec!["plain".into()];
    assert!(validate(&resource, &server).is_err());
}
