//! Provider initialization for model discovery and RLM command execution.
//!
//! [`models::execute`] loads providers for capability discovery. The RLM
//! provider-source implementation on [`crate::ProviderRegistry`] loads the same
//! application registry before applying the RLM selection policy.
pub mod models;
mod rlm;
