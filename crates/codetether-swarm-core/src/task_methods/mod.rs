//! Task construction, execution state and worktree intent decisions.
//!
//! Methods preserve the original builders, status transitions and intent policy.
use crate::*;
use chrono::Utc;
use uuid::Uuid;
mod sub_agent_construct;
mod sub_agent_lifecycle;
mod sub_task_builders;
mod sub_task_construct;
mod sub_task_lifecycle;
mod sub_task_worktree;
