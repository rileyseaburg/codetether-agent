//! Bind the four run routes to their native adapters.
pub(super) struct NativeHost;
impl codetether_run_cli::Host for NativeHost {
    type Ordinary = super::session_runtime::NativeHost;
    type Prd = crate::cli::go_ralph::AppHost;
    type Relay = super::relay_host::NativeHost;
    type Sol = super::sol_workflow::NativeHost;
}
