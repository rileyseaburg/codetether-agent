//! Bounded backward history loading and stable message-boundary selection.
//!
//! Implement HistorySource and pass a Request to load. Page contains the
//! preceding chronological messages and the boundary for the next request.
//! Fingerprint helpers and page selection also work without a storage backend.
pub mod anchor;
mod indexed;
mod load;
pub mod select;
mod source;
pub mod store;
mod types;
pub use load::run as load;
pub use source::HistorySource;
pub use types::{Fingerprint, Page, PageResult, Request};
#[cfg(test)]
mod tests;

/// Interactive page requests, state transitions and viewport preservation.
pub mod interactive;
