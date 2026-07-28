//! Visual diff module.

pub mod compare;
pub mod mask;
pub mod output;
pub mod stats;

#[cfg(test)]
mod tests;

pub use compare::{compare_rasters, DiffResult};
pub use mask::{build_diff_mask, DiffMask};
pub use output::diff_image;
pub use stats::DiffStats;
