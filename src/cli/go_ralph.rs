//! Application bindings for the dedicated autonomous Ralph pipeline.
mod execute;
mod generation;
mod host;
mod registry;
pub use codetether_go_prd::generate_prd_from_task;
pub use codetether_go_ralph::{GoRalphResult, StoryResult, format_go_ralph_result};
pub use execute::execute_go_ralph;
pub use generation::generate_prd_with_model_retry;

pub(crate) use host::AppHost;
mod cli_host;
