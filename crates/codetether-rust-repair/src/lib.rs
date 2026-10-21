//! Bounded tool-free repair proposals for editor-owned snapshots.
//!
//! [`run`] reads one JSON request and prints one validated proposal. It does not
//! edit project files or load agent sessions, memory, or history.
mod completion;
mod output_types;
mod paths;
mod prompt;
mod proposal;
mod request;
mod runtime;
mod types;
pub use completion::{completion_request, propose};
pub use output_types::{Edit, Proposal};
pub use paths::validate_path;
pub use proposal::{parse_proposal, validate_proposal};
pub use request::{MAX_REQUEST_BYTES, parse_request, validate_request};
pub use runtime::run;
pub use types::{Diagnostic, FileSnapshot, RepairRequest};

/// Provider selection; unset uses the configured default model.
/// # Examples
/// ```
/// let args = codetether_rust_repair::RustRepairArgs { model: None };
/// assert!(args.model.is_none());
/// ```
#[derive(clap::Args, Clone, Debug)]
pub struct RustRepairArgs {
    /// Optional provider/model override for this proposal.
    #[arg(long)]
    pub model: Option<String>,
}
