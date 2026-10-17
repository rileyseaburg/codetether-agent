//! Shared benchmark contracts, discovery, cost estimation, and scoring.
//! The application supplies provider loading and isolated agent execution.
mod detect_tier;
mod discover_prds;
pub mod metrics;
mod parse_model;
pub mod pricing;
mod types;
pub use detect_tier::detect_tier;
pub use discover_prds::discover_prds;
pub use parse_model::parse_model;
pub use types::*;
/// HTTP pricing lookup and best-effort result publication.
pub mod http;
pub mod quality;
pub mod suite;
pub mod workspace;
