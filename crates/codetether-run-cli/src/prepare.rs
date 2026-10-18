//! Prepare message, policy and workspace context before routing.
use super::prepared::Prepared;
use codetether_cli_options::RunArgs;
pub(super) async fn prepare(args: RunArgs) -> anyhow::Result<Prepared> {
    let message = codetether_run_context::with_attachments(args.message.trim(), &args.file)?;
    let jsonl_output = codetether_run_jsonl::enabled(&args.format);
    codetether_run_resume::validate_auto_continue(args.auto_continue_until)?;

    if message.is_empty() {
        anyhow::bail!("You must provide a message");
    }

    tracing::info!("Running with message: {}", message);

    codetether_swarm_cli::prepare_speculative(&args, &message, jsonl_output)?;

    // Load configuration
    let config = codetether_run_policy::load(&args).await;
    let workspace_dir = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let knowledge_snapshot = codetether_run_context::refresh_knowledge(&workspace_dir).await;

    Ok(Prepared {
        args,
        message,
        jsonl_output,
        config,
        workspace_dir,
        knowledge_snapshot,
    })
}
