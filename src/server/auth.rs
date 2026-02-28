//! Mandatory authentication middleware
//!
//! All endpoints except `/health` require a valid Bearer token.
//! **Auth cannot be disabled.** If no `CODETETHER_AUTH_TOKEN` is set the
//! server generates a secure random token at startup and prints it to stderr
//! so the operator can copy it — but the gates never open without a token.
//!
//! JWT support: If the Bearer token is a JWT, topic claims are extracted
//! and stored in request extensions for use by the bus stream endpoint.

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Paths that are exempt from authentication.
const PUBLIC_PATHS: &[&str] = &["/health"];

/// JWT claims extracted from the Bearer token for topic filtering.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Allowed topics for bus stream filtering.
    #[serde(default)]
    pub topics: Vec<String>,
    /// Optional user identifier.
    #[serde(default, rename = "sub")]
    pub subject: Option<String>,
    /// Additional scopes from the JWT.
    #[serde(default)]
    pub scopes: Vec<String>,
}

/// Parse a JWT token and extract claims from the payload.
/// Returns None if the token is not a valid JWT (e.g., it's a static token).
pub fn extract_jwt_claims(token: &str) -> Option<JwtClaims> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        // Not a JWT - it's likely a static token
        return None;
    }

    // Decode the payload (second part)
    let payload = URL_SAFE_NO_PAD.decode(parts[1]).ok()?;

    // Parse JSON
    let claims: JwtClaims = serde_json::from_slice(&payload).ok()?;

    Some(claims)
}

/// Shared auth state.
#[derive(Debug, Clone)]
pub struct AuthState {
    /// The required Bearer token.
    token: Arc<String>,
}

impl AuthState {
    /// Build from the environment.  If `CODETETHER_AUTH_TOKEN` is not set a
    /// 32-byte hex token is generated and logged once.
    pub fn from_env() -> Self {
        let token = match std::env::var("CODETETHER_AUTH_TOKEN") {
            Ok(t) if !t.is_empty() => {
                tracing::info!("Auth token loaded from CODETETHER_AUTH_TOKEN");
                t
            }
            _ => {
                let generated: String = {
                    let mut rng = rand::rng();
                    (0..32)
                        .map(|_| format!("{:02x}", rng.random::<u8>()))
                        .collect()
                };
                tracing::warn!(
                    token = %generated,
                    "No CODETETHER_AUTH_TOKEN set — generated a random token. \
                     Set CODETETHER_AUTH_TOKEN to use a stable token."
                );
                generated
            }
        };
        Self {
            token: Arc::new(token),
        }
    }

    /// Create with an explicit token (for tests).
    #[cfg(test)]
    pub fn with_token(token: impl Into<String>) -> Self {
        Self {
            token: Arc::new(token.into()),
        }
    }

    /// Return the active token (for display at startup).
    pub fn token(&self) -> &str {
        &self.token
    }
}

/// Axum middleware layer that enforces Bearer token auth on every request
/// except public paths.
pub async fn require_auth(mut request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let path = request.uri().path();

    // Allow public paths through without auth.
    if PUBLIC_PATHS.contains(&path) {
        return Ok(next.run(request).await);
    }

    // Extract the AuthState from extensions (set by the server setup).
    let auth_state = request
        .extensions()
        .get::<AuthState>()
        .cloned()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Extract Bearer token from Authorization header.
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok());

    let provided_token = match auth_header {
        Some(value) if value.starts_with("Bearer ") => &value[7..],
        _ => {
            // Also accept token via query parameter for SSE/WebSocket clients.
            let query = request.uri().query().unwrap_or("");
            let token_param = query.split('&').find_map(|pair| {
                let mut parts = pair.splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some("token"), Some(v)) => Some(v),
                    _ => None,
                }
            });
            match token_param {
                Some(t) => t,
                None => return Err(StatusCode::UNAUTHORIZED),
            }
        }
    };

    // Constant-time comparison to prevent timing attacks.
    if constant_time_eq(provided_token.as_bytes(), auth_state.token.as_bytes()) {
        // Extract JWT claims and add to request extensions for downstream handlers
        let claims = extract_jwt_claims(provided_token);
        if let Some(claims) = claims {
            request.extensions_mut().insert(claims);
        }
        Ok(next.run(request).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

/// Constant-time byte comparison.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_time_eq_works() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"short", b"longer"));
    }

    #[test]
    fn auth_state_generates_token_when_env_missing() {
        // Ensure the env var is not set for this test.
        // SAFETY: This is a single-threaded test; no other thread reads this env var.
        unsafe {
            std::env::remove_var("CODETETHER_AUTH_TOKEN");
        }
        let state = AuthState::from_env();
        assert_eq!(state.token().len(), 64); // 32 bytes = 64 hex chars
    }
}
