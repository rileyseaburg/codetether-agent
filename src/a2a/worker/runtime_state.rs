//! Native HTTP and bus specializations of shared task runtime state.
pub(super) use codetether_a2a_worker_core::dispatch::TaskReservation;
pub(super) type WorkerTaskRuntime = codetether_a2a_worker_core::initialization::WorkerTaskRuntime<
    reqwest::Client,
    crate::bus::AgentBus,
>;
