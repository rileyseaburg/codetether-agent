//! Native CLI, HTTP, bus and peer specializations of shared worker startup state.
pub type WorkerContext = codetether_a2a_worker_core::initialization::WorkerContext<
    codetether_cli_options::A2aArgs,
    reqwest::Client,
    codetether_bus_core::AgentBus,
    codetether_a2a_http::peer_handle::A2APeerHandle,
>;
