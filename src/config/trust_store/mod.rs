//! Project trust records keyed by canonical workspace path.
//!
//! The trust store keeps project-local policy decisions out of the repository
//! while still allowing trusted workspaces to opt into policy-bearing config.

mod record;
mod root;
mod status;
mod store;
mod workspace;

pub use status::ProjectTrustStatus;
pub use store::ProjectTrustStore;

#[cfg(test)]
mod tests;
