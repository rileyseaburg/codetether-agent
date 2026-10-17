//! Serializable Ralph contracts, grouped by the data each represents.
//!
//! Prd owns UserStory and QualityChecks values; RalphState records execution
//! progress with RalphStatus and ProgressEntry. VerificationStep describes
//! checks without executing them.
use crate::defaults::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
mod verification_step;
pub use verification_step::VerificationStep;
mod user_story;
pub use user_story::UserStory;
mod prd;
pub use prd::Prd;
mod quality_checks;
pub use quality_checks::QualityChecks;
mod ralph_state;
pub use ralph_state::RalphState;
mod ralph_status;
pub use ralph_status::RalphStatus;
mod progress_entry;
pub use progress_entry::ProgressEntry;
mod ralph_config;
pub use ralph_config::RalphConfig;
