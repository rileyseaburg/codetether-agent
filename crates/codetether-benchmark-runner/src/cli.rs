//! Execute CLI benchmark requests with the application-bound suite runner.
use crate::{Application, BenchmarkRunner};

#[async_trait::async_trait]
impl<H: Application> codetether_benchmark_cli::Runner for BenchmarkRunner<H> {
    async fn run(
        config: codetether_benchmark::BenchmarkConfig,
    ) -> anyhow::Result<codetether_benchmark::BenchmarkSuiteResult> {
        let runner = BenchmarkRunner::<H>::new(config);
        runner.run().await
    }
}
