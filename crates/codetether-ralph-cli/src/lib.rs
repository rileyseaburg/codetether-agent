//! Ralph CLI command policy independent of native loop execution.
//!
//! Host supplies Vault initialization and the existing Ralph loop. PRD schema,
//! selection precedence, configuration and status formatting live here.
mod create;
mod loop_runner;
pub use loop_runner::LoopRunner;
pub mod interactive;
mod output;
mod run;
mod selection;
mod status;
pub use output::print_result;
pub use selection::select_provider;
/// Native provider-loading and Ralph-loop execution boundary.
/// # Implementors
/// Applications preserve their runtime integration here.
#[async_trait::async_trait(?Send)]
pub trait Host {
    /// Load providers before applying the shared selection policy.
    /// # Errors
    /// Returns initialization or provider-selection failures.
    async fn load_provider(
        model: Option<String>,
    ) -> anyhow::Result<(
        std::sync::Arc<dyn codetether_provider_core::Provider>,
        String,
    )>;
    /// Run the native loop and print its selected result representation.
    /// # Errors
    /// Returns loop initialization, execution or serialization failures.
    async fn execute_loop(
        prd: std::path::PathBuf,
        provider: std::sync::Arc<dyn codetether_provider_core::Provider>,
        model: String,
        config: codetether_ralph_core::RalphConfig,
        json: bool,
    ) -> anyhow::Result<()>;
}
/// Execute a Ralph CLI operation.
/// # Arguments
/// * `args` - PRD action and run settings.
/// # Returns
/// Unit after output or loop execution.
/// # Errors
/// Returns invalid action, storage, host or serialization failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_ralph_cli::Host>(args: codetether_cli_options::RalphArgs) {
/// let _ = codetether_ralph_cli::execute::<H>(args).await;
/// # }
/// ```
pub async fn execute<H: Host>(args: codetether_cli_options::RalphArgs) -> anyhow::Result<()> {
    match args.action.as_str() {
        "create-prd" => create::execute(args).await,
        "status" => status::execute(args).await,
        "run" => run::execute::<H>(args).await,
        _ => anyhow::bail!(
            "Unknown action: {}. Use run, status, or create-prd",
            args.action
        ),
    }
}
