//! Invoke the shared relay with the native host.
use super::NativeHost;

pub(in crate::cli::run) async fn run_protocol_first_relay(
    agent_count: usize,
    task: &str,
    model_ref: &str,
    okr_id: Option<uuid::Uuid>,
    okr_run_id: Option<uuid::Uuid>,
) -> anyhow::Result<codetether_relay_report::AutochatCliResult> {
    codetether_cli_relay::run_protocol_first_relay::<NativeHost>(
        agent_count,
        task,
        model_ref,
        okr_id,
        okr_run_id,
    )
    .await
}
