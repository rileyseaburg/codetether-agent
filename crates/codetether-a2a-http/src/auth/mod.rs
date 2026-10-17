//! Bearer-token middleware for the A2A peer HTTP surface.
//!
//! [require_a2a_auth] preserves the optional configured-token policy for this
//! peer surface and accepts the process-local discovery capability. The two
//! agent-card paths stay public. This is separate from the application's
//! mandatory server authentication middleware.
//!
//! ```rust
//! use codetether_a2a_http::auth::constant_time_eq;
//! assert!(constant_time_eq(b"same", b"same"));
//! assert!(!constant_time_eq(b"same", b"other"));
//! ```
mod cached;
mod middleware;
mod token;
pub use middleware::require_a2a_auth;
pub use token::constant_time_eq;
