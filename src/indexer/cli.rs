//! cli responsibilities for codebase indexing.

use super::*;
mod run;
pub use run::run;

mod options;
mod print;
mod text_summary;
use options::options;
use print::print_summary;
use text_summary::text_summary;
