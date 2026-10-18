//! Bind the four run routes to their native adapters.
pub(super) struct NativeHost;
impl codetether_run_cli::Host for NativeHost {
    type Ordinary = codetether_run_runtime::application::ApplicationHost<crate::session::Session>;
    type Prd = crate::cli::go_ralph::AppHost;
    type Relay = codetether_cli_relay::application::ApplicationHost<
        crate::provider::ProviderRegistry,
        crate::session::Session,
    >;
    type Sol = super::sol_workflow::NativeHost;
}
