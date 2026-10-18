//! Preserve authenticated claim fields and default identity labels.
use crate::PolicyUser;
use codetether_http_auth::JwtClaims;

pub(super) fn from_claims(claims: &JwtClaims) -> PolicyUser {
    PolicyUser {
        user_id: claims
            .subject
            .clone()
            .unwrap_or_else(|| "bearer-token-user".to_string()),
        roles: claims.roles.clone(),
        tenant_id: claims.tenant_id.clone(),
        scopes: claims.scopes.clone(),
        auth_source: claims
            .auth_source
            .clone()
            .unwrap_or_else(|| "jwt".to_string()),
    }
}
