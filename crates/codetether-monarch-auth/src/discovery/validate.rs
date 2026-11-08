//! Enforce exact Monarch endpoints and required OAuth capabilities.
use super::models::{ResourceMetadata, ServerMetadata};
use super::{AUTHORIZE, ISSUER, REGISTER, RESOURCE, REVOKE, TOKEN};
use anyhow::{Result, ensure};

pub(super) fn validate(resource: &ResourceMetadata, server: &ServerMetadata) -> Result<()> {
    ensure!(
        resource.resource == RESOURCE && resource.authorization_servers == [ISSUER],
        "Untrusted Monarch resource metadata"
    );
    ensure!(
        ["mcp:read", "mcp:write"]
            .iter()
            .all(|s| resource.scopes_supported.iter().any(|v| v == s)),
        "Unsupported Monarch scopes"
    );
    // Never send tokens to an endpoint taken from untrusted metadata.
    ensure!(
        server.issuer == ISSUER
            && server.authorization_endpoint == AUTHORIZE
            && server.token_endpoint == TOKEN
            && server.registration_endpoint == REGISTER
            && server.revocation_endpoint == REVOKE,
        "Untrusted Monarch authorization metadata"
    );
    for (values, required) in [
        (&server.code_challenge_methods_supported, "S256"),
        (&server.token_endpoint_auth_methods_supported, "none"),
        (&server.response_types_supported, "code"),
        (&server.grant_types_supported, "authorization_code"),
        (&server.grant_types_supported, "refresh_token"),
    ] {
        ensure!(
            values.iter().any(|v| v == required),
            "Unsupported Monarch OAuth capabilities"
        );
    }
    Ok(())
}
