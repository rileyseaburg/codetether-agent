//! HTTP bearer authentication state, token extraction and middleware.
//!
//! AuthState carries a shared token and fixed public paths. require_auth
//! validates requests before exposing decoded claims to downstream handlers.
mod claims;
mod middleware;
mod state;
#[cfg(test)]
mod tests;
mod token;
mod util;

pub use claims::JwtClaims;
pub use middleware::require_auth;
pub use state::AuthState;
pub use token::extract_unverified_jwt_claims;

pub use state::{default_public_paths, generate_token, load_token};
pub use util::{bearer_token, constant_time_eq, provided_token, query_token};
