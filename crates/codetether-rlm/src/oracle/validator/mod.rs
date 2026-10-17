//! Trace routing, confidence configuration, and batch validation.

use super::grep_oracle::GrepOracle;
use super::schema::FinalPayload;

pub use super::ast_validation::validate_ast_payload;

pub use super::batch::{BatchValidationStats, SplitWriteStats};

pub use super::consensus::{build_base_trace, validate_with_consensus};

#[allow(unused_imports)]
pub use super::consensus_helpers::build_placeholder_trace;

pub use super::grep_validation::validate_grep_payload;

#[allow(unused_imports)]
pub use super::record::OracleTraceRecord;

pub use super::trace_types::{OracleResult, ValidatedTrace};

pub use super::types::{TraceStep, VerificationMethod};
mod batch;
mod config;
mod configure;
mod consensus;
mod fallback;
mod grep;
mod plain;
mod route;
mod state;
mod validate;
pub use config::Config;
pub use state::TraceValidator;
