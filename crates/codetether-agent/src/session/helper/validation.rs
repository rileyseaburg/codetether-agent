//! Native touched-file validation services.
pub(crate) use codetether_native_validation::NativeHost as ApplicationValidation;
pub use codetether_native_validation::{
    ValidationReport, build_validation_report, capture_git_dirty_files, track_touched_files,
};
