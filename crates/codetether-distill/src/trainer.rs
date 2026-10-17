//! LoRA training configuration and entry point.
//!
//! [train_lora] currently selects an adapter version and returns placeholder
//! metrics. It does not train a model or write an adapter.
mod config;
mod result;
mod run;
mod version;
pub use config::TrainConfig;
pub use result::TrainResult;
pub use run::train_lora;
