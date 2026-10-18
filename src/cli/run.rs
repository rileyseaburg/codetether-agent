//! Non-interactive run command

mod files;
mod jsonl;
mod knowledge;
mod relay_host;
mod run_execute;
mod session_runtime;
mod session_source;
mod sol_workflow;
use relay_host::run_protocol_first_relay;

use super::RunArgs;
use anyhow::Result;
use codetether_run_command::resolve_autochat_model;

pub use run_execute::execute;

pub(super) async fn execute_inner(args: RunArgs) -> Result<()> {
    let message = files::with_attachments(args.message.trim(), &args.file)?;
    let jsonl_output = jsonl::enabled(&args.format);
    super::run_checkpoint::validate_auto_continue(args.auto_continue_until)?;

    if message.is_empty() {
        anyhow::bail!("You must provide a message");
    }

    tracing::info!("Running with message: {}", message);

    codetether_swarm_cli::prepare_speculative(&args, &message, jsonl_output)?;

    // Load configuration
    let config = super::run_config::load(&args).await;
    let workspace_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let knowledge_snapshot = knowledge::refresh(&workspace_dir).await;

    // Protocol-first relay aliases in CLI:
    // - /go [count] <task>
    // - /autochat [count] <task>
    if let Some(request) = codetether_run_command::parse_relay_request(&message, jsonl_output)? {
        let codetether_run_command::RelayRequest {
            agent_count,
            task,
            require_prd,
            easy_go_requested,
            explicit_count,
        } = request;

        let model = resolve_autochat_model(
            args.model.as_deref(),
            std::env::var("CODETETHER_DEFAULT_MODEL").ok().as_deref(),
            config.default_model.as_deref(),
            easy_go_requested,
        );

        // PRD-gated execution path (mandatory for /go and default for /autochat)
        if require_prd {
            return codetether_go_cli::execute::<super::go_ralph::AppHost>(
                codetether_go_cli::Input {
                    task,
                    model,
                    agent_count,
                    easy_go_requested,
                    explicit_count,
                    format: args.format,
                },
            )
            .await;
        }

        // Explicit tactical /autochat path (requires --no-prd)
        let relay_result = run_protocol_first_relay(agent_count, &task, &model, None, None).await?;
        codetether_run_report::render_relay(&args.format, &relay_result)?;
        return Ok(());
    }

    // Create or continue session.
    codetether_run_runtime::execute::<session_runtime::NativeHost>(codetether_run_runtime::Input {
        args,
        config,
        workspace_dir,
        knowledge_snapshot,
        message,
        jsonl_output,
    })
    .await
}
