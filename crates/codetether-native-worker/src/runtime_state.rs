//! Native HTTP and bus specializations of shared task runtime state.
pub use codetether_a2a_worker_core::dispatch::TaskReservation;
pub type WorkerTaskRuntime = codetether_a2a_worker_core::initialization::WorkerTaskRuntime<
    reqwest::Client,
    codetether_bus_core::AgentBus,
>;
