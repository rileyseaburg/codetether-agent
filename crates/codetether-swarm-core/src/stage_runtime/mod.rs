//! Dependency-stage traversal with host-supplied runtime operations.
//!
//! [execute_stages] owns readiness checks, stage ordering and failed-dependency
//! gating. Implement [StageRuntime] for backend execution, events and publication.
use crate::{SubTask, SubTaskResult};
use anyhow::Result;
use async_trait::async_trait;
use std::{collections::HashSet, time::Instant};
mod contract;
mod execute;
mod input;
mod stage;
pub use contract::StageRuntime;
pub use execute::execute_stages;
pub use input::StageInput;
