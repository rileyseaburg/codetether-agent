//! Shared claim, timeout, execution-outcome and release lifecycle.
//!
//! [handle_task] sequences task handling through [TaskHandlerHost], while the
//! host retains transport and application execution state.
use crate::{
    dispatch::execute_with_timeout,
    task_config::{task_metadata, task_str, task_timeout_secs},
    task_timeline,
};
use anyhow::Result;
mod host;
mod run;
pub use host::TaskHandlerHost;
pub use run::handle_task;
