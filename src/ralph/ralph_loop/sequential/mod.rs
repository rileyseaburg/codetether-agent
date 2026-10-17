//! Sequential story iteration, prompt construction and completion handling.
//!
//! The group owns its application execution flow; shared policy and scheduling
//! remain in ralph-core. Runtime resources retain their existing lifetimes.
use super::*;
mod driver;
mod scheduling_sequential;
mod seq_error;
mod seq_failure;
mod seq_iteration;
mod seq_prompt;
mod seq_response;
mod seq_success;
mod seq_unchecked;
