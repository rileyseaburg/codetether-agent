//! Persisted context strategies and model-relative budget resolution.
//!
//! [DerivePolicy] describes the strategy. [policy_implicit] resolves legacy
//! selections, and budget modules cap working context using shared model limits.
use serde::{Deserialize, Serialize};
mod policy;
pub use policy::DerivePolicy;
/// Completion limit read from the session environment setting.
pub mod completion_limit;
/// Incremental budget override and zero-sentinel resolution.
pub mod incremental_budget_config;
/// Safe model input budgets after output and protocol reserves.
pub mod input_budget;
/// Request tuning fields independent of provider execution.
pub mod options;
/// Pure implicit selection based on persisted policy and resolved inputs.
pub mod policy_implicit;
/// Reset threshold precedence and model-relative fractions.
pub mod reset_threshold;

/// Incremental selection preparation and projection assembly.
pub mod incremental;

/// Reset preparation and summary reconstruction.
pub mod reset;

/// Effective policy resolution with a host-owned warning hook.
pub mod effective;

/// Concrete execution choice with keep-last recovery precedence.
pub mod dispatch;
