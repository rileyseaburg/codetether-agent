//! Planning-mode state and executable entry/exit tools.
//!
//! [PlanEnterTool] starts a plan; [PlanExitTool] marks progress and renders a
//! status report. Both share one process-local plan and activation flag.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
mod enter;
mod enter_construct;
mod enter_report;
mod enter_tool;
mod exit;
mod exit_construct;
mod exit_tool;
mod params;
mod snapshot;
mod state;
mod step_report;
pub use enter_construct::PlanEnterTool;
pub use exit_construct::PlanExitTool;
use params::{EnterParams, ExitParams};
use state::{Plan, PlanStep};

static IN_PLAN_MODE: AtomicBool = AtomicBool::new(false);

lazy_static::lazy_static! {
    static ref CURRENT_PLAN: RwLock<Option<Plan>> = RwLock::new(None);
}
