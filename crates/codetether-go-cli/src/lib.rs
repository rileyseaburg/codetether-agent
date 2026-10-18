//! PRD-backed CLI objective approval and execution.
//!
//! Supply validated [Input] and an application [Host] to [execute]. Drafting,
//! approval and pipeline execution are separated to preserve their ordering.
mod approval;
mod draft;
mod execute;
mod host;
mod input;
mod pipeline;
pub use execute::execute;
pub use host::Host;
pub use input::Input;
