//! Process-local task acceptance and ordered bus notification.
pub mod contract;
mod service;
pub use service::enqueue;
#[cfg(test)]
mod tests;
