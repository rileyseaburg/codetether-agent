//! Route prepared input to relay or ordinary-session execution.
use super::prepared::Prepared;
use crate::Host;
use codetether_cli_options::RunArgs;
pub(super) async fn execute<H: Host>(args: RunArgs) -> anyhow::Result<()>
where
    <H::Prd as codetether_go_ralph::Host>::Registry:
        codetether_autochat_core::model_rotation::ModelRegistry,
{
    let Prepared {
        args,
        message,
        jsonl_output,
        config,
        workspace_dir,
        knowledge_snapshot,
    } = crate::prepare::prepare(args).await?;
    if let Some(request) = codetether_run_command::parse_relay_request(&message, jsonl_output)? {
        return crate::relay::execute::<H>(args, config, request).await;
    }
    codetether_run_runtime::execute::<H::Ordinary>(codetether_run_runtime::Input {
        args,
        config,
        workspace_dir,
        knowledge_snapshot,
        message,
        jsonl_output,
    })
    .await
}
