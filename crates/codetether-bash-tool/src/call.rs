//! Per-call values shared by the execution and reporting modules.
use serde_json::Value;
use std::{path::PathBuf, time::Instant};
pub(super) struct Call<'a> {
    pub command: &'a str,
    pub cwd: Option<PathBuf>,
    pub effective_cwd: Option<PathBuf>,
    pub timeout_secs: u64,
    pub wrapped_command: String,
    pub exec_start: Instant,
    pub args: &'a Value,
}
