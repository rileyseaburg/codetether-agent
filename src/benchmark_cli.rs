//! Native benchmark runner binding for the shared CLI.
use crate::{benchmark, cli};
struct NativeRunner;
#[async_trait::async_trait]
impl codetether_benchmark_cli::Runner for NativeRunner {
    async fn run(
        config: benchmark::BenchmarkConfig,
    ) -> anyhow::Result<benchmark::BenchmarkSuiteResult> {
        let runner = benchmark::BenchmarkRunner::new(config);
        runner.run().await
    }
}
pub(super) async fn execute(args: cli::BenchmarkArgs) -> anyhow::Result<()> {
    codetether_benchmark_cli::execute::<NativeRunner>(args).await
}
