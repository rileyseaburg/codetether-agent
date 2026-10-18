//! Provider-backed tool selection and provider ownership.
//!
//! [create] clones the shared provider for summarization, RLM and recall, then
//! transfers the remaining provider to Ralph. The application supplies only the
//! concrete factory selected by [ProviderTool]. Unsupported catalog kinds leave
//! both the source and factory untouched.
pub mod bound;
mod create;
mod kind;
pub use create::create;
pub use kind::ProviderTool;
