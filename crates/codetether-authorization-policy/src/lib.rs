//! OPA and local role/scope authorization policy evaluation.
//!
//! check_policy evaluates PolicyUser and PolicyResource context using the
//! environment-selected mode. enforce_policy maps denial to HTTP forbidden.
//! The application authenticates requests before installing policy_middleware.
mod check;
mod client;
mod config;
mod local;
mod remote;
mod roles;
mod scope;
mod types;
mod wire;
pub use check::{check_policy, enforce_policy};
pub use types::{PolicyResource, PolicyUser};
#[cfg(test)]
mod tests;

/// Ordered route mapping and fail-closed permission lookup.
pub mod routes;

/// Authenticated HTTP request identity projection.
pub mod user;

mod middleware;
pub use middleware::policy_middleware;
