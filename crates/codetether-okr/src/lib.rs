//! Objectives, measurable key results, execution runs, and file persistence.
//!
//! Build an [`Okr`] with [`KeyResult`] values, validate it, and use an
//! [`OkrRepository`] to persist it. [`OkrRun`] tracks approval and execution.
//!
//! # Examples
//! ```
//! use codetether_okr::{Okr, KeyResult};
//! let mut okr = Okr::new("Reliability", "Improve delivery");
//! okr.add_key_result(KeyResult::new(okr.id, "Successful checks", 10.0, "count"));
//! assert!(okr.validate().is_ok());
//! assert_eq!(okr.progress(), 0.0);
//! ```

mod approval;
mod defaults;
mod key_result;
mod objective;
mod outcome;
pub mod persistence;
mod run;
mod validation;
pub use approval::{ApprovalChoice, ApprovalDecision};
pub use key_result::{KeyResult, KeyResultStatus, KrMetricType};
pub use objective::{Okr, OkrStatus};
pub use outcome::{KrOutcome, KrOutcomeType};
pub use persistence::OkrRepository;
pub use run::{OkrRun, OkrRunStatus};
pub use validation::OkrValidationError;
#[cfg(test)]
mod contract;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_model;
