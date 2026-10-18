//! Lookup and JSON presentation of accepted task completion receipts.
mod response;
mod service;
pub use service::snapshot;
#[cfg(test)]
mod tests;
