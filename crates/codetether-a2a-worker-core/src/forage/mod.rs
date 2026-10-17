//! Worker task metadata translated into typed forage options.
//!
//! [build_forage_options] applies metadata aliases and defaults. The resulting
//! [ForageTaskOptions] can be converted by a host to its command-line arguments.
use crate::task_config::{
    metadata_bool, metadata_f64, metadata_str, metadata_string_list, metadata_u64, metadata_usize,
};
use codetether_forage_core::settings::{
    normalize_forage_execution_engine, normalize_forage_swarm_strategy,
};
use std::path::PathBuf;
mod build;
mod fallback;
mod options;
pub use build::build_forage_options;
use fallback::NonEmptyFallback;
pub use options::ForageTaskOptions;
