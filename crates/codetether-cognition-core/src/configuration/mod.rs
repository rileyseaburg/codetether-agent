//! Runtime options and typed environment readers.
//!
//! [CognitionRuntimeOptions] supplies constructor defaults, while
//! [options_from_env] applies the established environment-specific defaults.
pub mod env;
mod from_env;
mod options;
pub use from_env::options_from_env;
pub use options::CognitionRuntimeOptions;
