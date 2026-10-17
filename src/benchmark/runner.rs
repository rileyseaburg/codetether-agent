//! Application adapter connecting shared benchmark scheduling to Ralph.
//!
//! BenchmarkRunner supplies provider loading and isolated PRD execution;
//! codetether_benchmark owns scheduling, scoring, workspace setup and transport.
use super::types::{BenchmarkConfig, BenchmarkSuiteResult};
mod backend;
mod configuration;
mod execution;
mod measurements;
mod prd;
mod quality;
mod result;
mod stories;
mod workspace;
/// Run benchmark suites using the application's provider registry and Ralph.
///
/// # Examples
/// ```
/// let runner = codetether_agent::benchmark::BenchmarkRunner::new(Default::default());
/// let _ = runner;
/// ```
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}
impl BenchmarkRunner {
    /// Construct a runner with the supplied discovery and execution settings.
    ///
    /// # Arguments
    /// * `config` - Discovery, execution limits, and output settings.
    /// # Returns
    /// A runner ready to execute the configured suite.
    /// # Examples
    /// ```
    /// let runner = codetether_agent::benchmark::BenchmarkRunner::new(Default::default());
    /// let _ = runner;
    /// ```

    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }
    /// Execute the suite and return the persisted benchmark report.
    ///
    /// # Returns
    /// The completed suite after persisting its report.
    /// # Errors
    /// Returns discovery, provider-loading, serialization or output-writing errors.
    /// # Examples
    /// ```no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let runner = codetether_agent::benchmark::BenchmarkRunner::new(Default::default());
    /// let report = runner.run().await?;
    /// assert_eq!(report.agent, "codetether");
    /// # Ok(()) }
    /// ```

    /// Run the full benchmark suite using the application's Ralph adapter.
    pub async fn run(&self) -> anyhow::Result<BenchmarkSuiteResult> {
        codetether_benchmark::suite::run_suite(
            &self.config,
            &backend::RunnerBackend(self),
            env!("CARGO_PKG_VERSION"),
        )
        .await
    }
}
