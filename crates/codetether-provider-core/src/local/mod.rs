//! Local-provider configuration, model catalog, prompt projection, and env parsing.
//!
//! [LocalCudaConfig] is shared across feature sets. [catalog] supplies the
//! existing local model list; [LocalCudaPrompt] formats request projections.
pub mod catalog;
mod config;
pub mod env;
mod prompt;
pub use config::LocalCudaConfig;
pub use prompt::LocalCudaPrompt;
