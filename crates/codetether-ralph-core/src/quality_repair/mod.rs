//! Quality-check, verification and repair sequencing for parallel stories.
//!
//! [finish] normalizes the initial model result and drives bounded retries
//! through [RepairHost]. The host owns shell access, verification, repair
//! execution and logging; [QualityProgress] holds only the evolving result.
use crate::QualityChecks;
use anyhow::Result;
mod finish;
mod host;
mod retry;
mod state;
pub use finish::finish;
pub use host::RepairHost;
pub use retry::run_quality_retries;
pub use state::QualityProgress;

mod evidence;
mod verify;
mod verify_without_checks;
pub use verify::verify_attempt;
pub use verify_without_checks::verify_without_checks;
