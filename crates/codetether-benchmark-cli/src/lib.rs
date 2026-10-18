//! Benchmark command validation, configuration and result presentation.
//!
//! The host supplies actual suite execution, including its application version.
mod config;
mod models;
mod output;
mod rankings;
/// Native benchmark execution interface.
/// # Implementors
/// Applications connect this to their provider and Ralph integration.
#[async_trait::async_trait]
pub trait Runner {
    /// Execute and persist a configured benchmark suite.
    /// # Errors
    /// Propagates initialization, execution or persistence failures.
    async fn run(
        config: codetether_benchmark::BenchmarkConfig,
    ) -> anyhow::Result<codetether_benchmark::BenchmarkSuiteResult>;
}
/// Validate options, execute through the host, and print the selected report.
/// # Arguments
/// * `args` - Parsed benchmark options.
/// # Returns
/// Unit after execution and report output.
/// # Errors
/// Returns missing-model, host execution or serialization failures.
/// # Examples
/// ```no_run
/// # async fn example<R: codetether_benchmark_cli::Runner>(args: codetether_cli_options::BenchmarkArgs) {
/// let _ = codetether_benchmark_cli::execute::<R>(args).await;
/// # }
/// ```
pub async fn execute<R: Runner>(args: codetether_cli_options::BenchmarkArgs) -> anyhow::Result<()> {
    if args.models.is_empty() {
        anyhow::bail!("At least one model is required (--models provider:model)");
    }
    let json = args.json;
    let config = config::build(args);
    let result = R::run(config).await?;
    output::print(&result, json)
}
