//! Swarm CLI configuration, decomposition selection and result reporting.
//!
//! Host retains global bus installation and native executor ownership.
mod config;
mod output;
mod strategy;
use codetether_swarm_core::{DecompositionStrategy, SwarmConfig, SwarmResult};
/// Native bus and executor construction for the swarm command.
/// # Implementors
/// Applications supply their global bus and execution runtime here.
#[async_trait::async_trait(?Send)]
pub trait Host {
    /// Bus created before configuration and executor construction.
    type Bus;
    /// Native executor configured for this command.
    type Executor;
    /// Create and install the live command bus.
    fn create_bus() -> Self::Bus;
    /// Construct the executor and attach the bus.
    fn create_executor(config: SwarmConfig, bus: Self::Bus) -> Self::Executor;
    /// Run the task with its selected strategy.
    /// # Errors
    /// Returns native orchestration/execution failures.
    async fn run(
        executor: &Self::Executor,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> anyhow::Result<SwarmResult>;
}
/// Execute one swarm command with the original initialization order.
/// # Arguments
/// * `args` - Task and execution options.
/// # Returns
/// Unit after result output.
/// # Errors
/// Propagates host execution and output serialization failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_swarm_cli::Host>(args: codetether_cli_options::SwarmArgs) {
/// let _ = codetether_swarm_cli::execute::<H>(args).await;
/// # }
/// ```
pub async fn execute<H: Host>(args: codetether_cli_options::SwarmArgs) -> anyhow::Result<()> {
    let bus = H::create_bus();
    let executor = H::create_executor(config::build(&args), bus);
    let strategy = strategy::resolve(&args.strategy);
    let result = H::run(&executor, &args.task, strategy).await?;
    output::print(&result, args.json)
}

mod speculative;
pub use speculative::prepare_speculative;
