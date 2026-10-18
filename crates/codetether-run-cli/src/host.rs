//! Native host types bound to the shared run workflows.
/// Runtime implementations used by run-command dispatch.
/// # Implementors
/// Applications bind their ordinary, PRD, relay and Sol host adapters.
pub trait Host {
    /// Ordinary session runtime.
    type Ordinary: codetether_run_runtime::Host;
    /// Approved PRD and Ralph runtime.
    type Prd: codetether_go_cli::Host;
    /// Protocol-first relay runtime.
    type Relay: codetether_cli_relay::Host;
    /// Tool-free Sol planning with a native coding worker.
    type Sol: codetether_sol_workflow::Host;
}
