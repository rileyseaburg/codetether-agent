//! Atomic task claiming and worker assignment responses.
mod request;
mod service;
pub use request::ClaimRequest;
pub use service::claim;
