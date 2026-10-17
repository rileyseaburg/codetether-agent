//! Shared PRD, verification, progress and execution-state contracts for Ralph.
//!
//! Build or load Prd values, select ready stories, and persist progress using
//! the inherent PRD methods. Runtime configuration and persistence contracts
//! are shared; agent execution stays with the application integration.
mod defaults;
mod prd_methods;
mod quality_checks;
mod types;
pub use types::*;
mod config_default_impl;
mod config_defaults;
pub mod presentation;
pub mod progress;
pub mod state_store;
pub mod store_http;
pub mod store_memory;
mod template;
pub mod verifiability;
pub mod verification;
pub use template::create_prd_template;
pub mod conflict_prompt;
pub mod events;
pub mod lifecycle;
pub mod quality;
pub mod rate_limit;
pub mod relay_profiles;

/// Host-driven quality verification and repair state machine.
pub mod quality_repair;

/// Host-driven sequential and dependency-stage orchestration.
pub mod scheduling;

/// Relay role mapping and calibrated provider ranking.
pub mod delegation;
