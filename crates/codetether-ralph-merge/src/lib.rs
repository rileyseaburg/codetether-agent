//! Integrate verified Ralph story output into the main workspace.
//!
//! [merge_parallel_story] commits an isolated story before merging, routes merge
//! outcomes, invokes conflict resolution and verifies completion. [Host] supplies
//! native commit policy, verification, state mutation and nonblocking reporting.
//! Cleanup and abort decisions remain specific to each original outcome branch.
mod conflict_complete;
mod conflict_invoke;
mod conflict_merge;
mod conflict_result;
mod conflict_verify;
mod conflicts;
mod context;
mod host;
mod merge_dispatch;
mod merge_nonconflict;
mod merge_story;
mod merge_success;
mod merge_verify;
mod merge_without_worktree;
use codetether_bus_core::AgentBus;
use codetether_provider_core::Provider;
use codetether_ralph_core::{RalphState, UserStory, events::RalphEvent};
use codetether_worktree::{MergeResult, WorktreeInfo, WorktreeManager};
use conflict_complete::complete_resolved_merge;
use conflict_invoke::invoke_merge_resolver;
use conflict_merge::resolve_parallel_merge;
use conflict_result::report_conflict_resolution;
use conflict_verify::verify_resolved_merge;
use conflicts::resolve_conflicts_static;
use context::MergeContext;
pub use host::Host;
use merge_dispatch::apply_worktree_merge;
use merge_nonconflict::report_nonconflict_merge;
pub use merge_story::merge_parallel_story;
use merge_success::complete_successful_merge;
use merge_verify::verify_merged_story;
use merge_without_worktree::complete_story_without_worktree;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tracing::{info, warn};
