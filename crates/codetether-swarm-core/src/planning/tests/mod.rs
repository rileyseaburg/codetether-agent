//! Completion-policy cases using an in-memory host; no provider I/O.
use super::{PlanCompletion, generate_plan};
use anyhow::Result;
use async_trait::async_trait;
mod fixture;
use fixture::{Mock, PARALLEL, ok};
mod initial_error;
mod initial_success;
mod optional_fallback;
mod parallel_failure;
mod repair_success;
