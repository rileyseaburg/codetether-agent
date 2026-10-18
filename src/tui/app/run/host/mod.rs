//! Native operations used by shared TUI startup sequencing.
use super::*;
use super::{session_outcome::SessionLoadOutcome, session_resolve::Resolved, startup::Startup};
use crate::{
    bus::AgentBus,
    config::AccessMode,
    provider::ProviderRegistry,
    session::{Session, TailLoad},
    tui::{app::state::App, models::WorkspaceSnapshot, worker_bridge::TuiWorkerBridge},
};
use codetether_tui_startup::Terminal;
use std::{path::Path, sync::Arc};
mod frame;
mod initial_frame;
mod load;
mod runtime;
pub(super) struct Native;
