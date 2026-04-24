use std::time::Duration;

use anyhow::Result;
use serde_json::Value;

use super::input::KilnPluginInput;
use super::join::join_error;
use super::runner::{self, KilnOutcome};

const DEFAULT_TIMEOUT_SECS: u64 = 10;
const MAX_TIMEOUT_SECS: u64 = 60;

pub enum KilnRunResult {
    Finished(Result<KilnOutcome>),
    Timeout(u64),
}

pub struct KilnRun {
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
    timeout_secs: u64,
}

impl KilnRun {
    pub fn new(source_name: String, source: String, input: KilnPluginInput) -> Self {
        Self {
            source_name,
            source,
            hook: input.hook,
            args: input.args,
            timeout_secs: input
                .timeout_secs
                .unwrap_or(DEFAULT_TIMEOUT_SECS)
                .min(MAX_TIMEOUT_SECS),
        }
    }
}

pub async fn run(request: KilnRun) -> Result<KilnRunResult> {
    let timeout = Duration::from_secs(request.timeout_secs);
    let task = tokio::task::spawn_blocking(move || {
        runner::run(
            request.source_name,
            request.source,
            request.hook,
            request.args,
        )
    });
    match tokio::time::timeout(timeout, task).await {
        Ok(joined) => Ok(KilnRunResult::Finished(joined.map_err(join_error)?)),
        Err(_) => Ok(KilnRunResult::Timeout(timeout.as_secs())),
    }
}
