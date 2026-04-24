use super::auth::JwtClaims;
use super::policy::PolicyUser;
use axum::{body::Body, http::Request};

pub fn from_request(request: &Request<Body>) -> PolicyUser {
    if let Some(claims) = request.extensions().get::<JwtClaims>() {
        return from_claims(claims);
    }
    if static_token_admin_enabled() {
        return static_user(vec!["admin".to_string()], "static_token_admin");
    }
    static_user(Vec::new(), "static_token")
}

fn from_claims(claims: &JwtClaims) -> PolicyUser {
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

fn static_user(roles: Vec<String>, auth_source: &str) -> PolicyUser {
    PolicyUser {
        user_id: "bearer-token-user".to_string(),
        roles,
        tenant_id: None,
        scopes: Vec::new(),
        auth_source: auth_source.to_string(),
    }
}

fn static_token_admin_enabled() -> bool {
    static ENABLED: std::sync::LazyLock<bool> =
        std::sync::LazyLock::new(|| super::env_bool("CODETETHER_STATIC_TOKEN_ADMIN", false));
    *ENABLED
}

#[cfg(test)]
mod tests;
