//! Choose PRD execution or a tactical relay after request validation.
use crate::Host;
use codetether_cli_options::RunArgs;
use codetether_cli_relay::run_protocol_first_relay;
use codetether_config::Config;
use codetether_run_command::RelayRequest;
pub(super) async fn execute<H: Host>(
    args: RunArgs,
    config: Config,
    request: RelayRequest,
) -> anyhow::Result<()>
where
    <H::Prd as codetether_go_ralph::Host>::Registry:
        codetether_autochat_core::model_rotation::ModelRegistry,
{
    let codetether_run_command::RelayRequest {
        agent_count,
        task,
        require_prd,
        easy_go_requested,
        explicit_count,
    } = request;

    let model = codetether_run_command::resolve_autochat_model(
        args.model.as_deref(),
        std::env::var("CODETETHER_DEFAULT_MODEL").ok().as_deref(),
        config.default_model.as_deref(),
        easy_go_requested,
    );

    // PRD-gated execution path (mandatory for /go and default for /autochat)
    if require_prd {
        return codetether_go_cli::execute::<H::Prd>(codetether_go_cli::Input {
            task,
            model,
            agent_count,
            easy_go_requested,
            explicit_count,
            format: args.format,
        })
        .await;
    }

    // Explicit tactical /autochat path (requires --no-prd)
    let relay_result =
        run_protocol_first_relay::<H::Relay>(agent_count, &task, &model, None, None).await?;
    codetether_run_report::render_relay(&args.format, &relay_result)?;
    return Ok(());
}
