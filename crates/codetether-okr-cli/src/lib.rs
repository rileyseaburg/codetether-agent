//! OKR command execution and presentation using the shared repository.
//!
//! execute opens the same configured repository before action dispatch.
//! Action and report modules preserve existing queries and output contracts.
use codetether_cli_options::OkrArgs;
use codetether_okr::OkrRepository;
mod create;
mod export;
mod list;
mod report;
mod runs;
mod stats;
mod status;
/// Execute the requested OKR operation.
/// # Arguments
/// * `args` - Action, filters and output options.
/// # Returns
/// Unit after repository operations and report output complete.
/// # Errors
/// Propagates repository, input, UUID and serialization failures.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_cli_options::OkrArgs) {
/// let _ = codetether_okr_cli::execute(args).await;
/// # }
/// ```
pub async fn execute(args: OkrArgs) -> anyhow::Result<()> {
    let repo = OkrRepository::from_config().await?;
    match args.action.as_str() {
        "list" => list::execute(args, &repo).await,
        "status" => status::execute(args, &repo).await,
        "create" => create::execute(args, &repo).await,
        "runs" => runs::execute(args, &repo).await,
        "export" => export::execute(args, &repo).await,
        "stats" => stats::execute(args, &repo).await,
        "report" => report::execute(args, &repo).await,
        _ => anyhow::bail!("Unknown OKR action: {}", args.action),
    }
}

mod list_render;
mod report_okr;
mod report_okr_keys;
mod report_okr_render;
mod report_okr_runs;
mod report_run;
mod report_run_text;
mod status_render;
mod status_runs;
