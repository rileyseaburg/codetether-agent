//! Touched-file validation planning and diagnostic continuation reports.
//!
//! [track_touched_files] records structured edits; [validation_scope] never
//! widens that set with unrelated dirty files. [build_validation_report] runs
//! refactor checks first, then collects and formats per-file diagnostics through
//! a native [Host]. No validation commands run merely by constructing this API.
mod build;
mod collect;
mod diagnostics;
mod git;
mod host;
mod input_paths;
mod metadata;
mod paths;
mod render;
mod report;
mod scope;
mod touched;
pub use build::build_validation_report;
pub use git::capture_git_dirty_files;
pub use host::Host;
pub use report::ValidationReport;
pub use scope::validation_scope;
pub use touched::track_touched_files;
#[cfg(test)]
mod tests;

/// Shared ESLint command preparation and output decoding.
pub mod eslint;
