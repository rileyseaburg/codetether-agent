//! Bind a spawned child job to its parent's shared runtime capability.

use super::job::Job;
use crate::swarm::SubTaskResult;

pub(super) async fn execute(job: Job) -> (String, anyhow::Result<SubTaskResult>) {
    match job.control.clone() {
        Some(control) => control.scope(super::job_execute::execute(job)).await,
        None => super::job_execute::execute(job).await,
    }
}
