//! Reusable worker startup identity, workspace parsing and processing state.
//! [parse_codebases] retains configured workspace order. [resolve_name] binds
//! the runtime identity. [processing_set] creates the shared active-task set.
mod codebases;
mod name;
mod processing;
pub use codebases::parse_codebases;
pub use name::resolve_name;
pub use processing::{ProcessingSet, processing_set};
