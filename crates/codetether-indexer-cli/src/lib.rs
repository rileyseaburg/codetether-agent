//! Indexer CLI options, output persistence and summary presentation.
//!
//! [IndexArgs] describes one indexing operation. [run] normalizes its options,
//! builds through a supplied [EmbeddingResolver], writes the resulting index,
//! and prints either JSON or a human-readable summary.
use anyhow::Result;
pub use codetether_indexer::EmbeddingResolver;
use codetether_indexer::{
    BuildOptions, CodebaseIndex, build_index_with_resolver, default_index_path,
};
use std::path::{Path, PathBuf};
mod args;
mod options;
mod print;
mod run;
mod text_summary;
pub use args::IndexArgs;
use options::options;
use print::print_summary;
pub use run::run;
use text_summary::text_summary;
