//! Provider initialization for model discovery, search and autonomous commands.
//!
//! [`models::execute`] loads providers for capability discovery. The RLM
//! provider-source implementation on [`crate::ProviderRegistry`] loads the same
//! application registry before applying the RLM selection policy.
pub mod models;
pub mod ralph;
mod rlm;
pub mod search;
