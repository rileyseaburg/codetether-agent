//! Native job execution used by the shared local-stage adapter.
use codetether_swarm_local_job as job;
mod job_host;
pub(super) mod job_run;
pub(super) use codetether_swarm_runtime::local_stage::execute;
