//! Harvest training records from successful runs.
//!
//! [HarvestFilter] selects accepted records; [harvest_from_dir] reads JSONL
//! files under the supplied data directory's sft child.
mod filter;
mod parse;
mod read;
mod record;
pub use filter::HarvestFilter;
pub use read::harvest_from_dir;
pub use record::TrainingRecord;
