//! Native CLI, HTTP, bus and peer specializations of shared worker startup state.
pub(super) type WorkerContext = codetether_a2a_worker_core::initialization::WorkerContext<
    crate::cli::A2aArgs,
    reqwest::Client,
    crate::bus::AgentBus,
    crate::a2a::spawn::A2APeerHandle,
>;
