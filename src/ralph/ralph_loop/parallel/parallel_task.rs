//! Native provider bindings for the shared owned parallel task.
use super::host::Native;
pub(in crate::ralph::ralph_loop) type ParallelTask = codetether_ralph_task::Task<Native>;
