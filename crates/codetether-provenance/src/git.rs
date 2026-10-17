//! Provenance-aware commit messages and Git execution adapters.

mod asynchronous;
mod blocking;
mod context;
mod message;
mod repository;
mod signing;
pub use asynchronous::git_commit_with_provenance;
pub use blocking::git_commit_with_provenance_blocking;
pub use message::commit_message_with_provenance;
pub use repository::enrich_from_repo;
pub use signing::commit_args_with_signature_policy;
#[cfg(test)]
mod tests;
