//! Prepare and announce speculative branches requested by a run.
use codetether_cli_options::RunArgs;
/// Build speculative branch plans and emit the existing CLI notices.
/// # Arguments
/// * `args` - Branch count and requested strategies.
/// * `message` - User task used to build branch plans.
/// * `jsonl_output` - Suppress the plain-text notice for JSONL.
/// # Returns
/// Unit after optional planning and output.
/// # Errors
/// Returns branch-plan construction errors.
/// # Examples
/// ```no_run
/// # fn example(args: &codetether_cli_options::RunArgs) -> anyhow::Result<()> {
/// codetether_swarm_cli::prepare_speculative(args, "task", false)?;
/// # Ok(()) }
/// ```
pub fn prepare_speculative(
    args: &RunArgs,
    message: &str,
    jsonl_output: bool,
) -> anyhow::Result<()> {
    if args.branches > 1 {
        let runner = codetether_swarm_core::collapse::speculative::SpeculativeRunner::new(
            args.branches,
            args.strategies.clone(),
        );
        let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let specs = runner.build_branches(&cwd, &message)?;
        tracing::info!(branches = %runner.branch_count, "Many-worlds speculative mode enabled");
        for spec in &specs {
            tracing::info!(
                branch = %spec.branch_name,
                strategy = %spec.strategy_prompt,
                "Speculative branch assigned"
            );
        }
        if !jsonl_output {
            println!(
                "[speculative] {} parallel branches queued (collapse controller will race + prune)",
                runner.branch_count
            );
        }
        // TODO: wire through SwarmExecutor when parallel dispatch is ready
    }

    Ok(())
}
