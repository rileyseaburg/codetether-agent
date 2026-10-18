//! Native services used by shared claimed-task sequencing.
mod events;
mod sessions;
mod special;
pub(super) struct Native<'a>(pub &'a super::super::WorkerTaskRuntime);
