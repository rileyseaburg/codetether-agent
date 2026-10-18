//! Benchmark configuration from the existing CLI option fields.
use codetether_cli_options::BenchmarkArgs;
pub(super) fn build(args: BenchmarkArgs) -> codetether_benchmark::BenchmarkConfig {
    let config = codetether_benchmark::BenchmarkConfig {
        prd_dir: args.prd_dir,
        models: args.models,
        tier: args.tier,
        parallel: args.parallel,
        max_iterations: args.max_iterations,
        story_timeout_secs: args.story_timeout,
        output: args.output,
        cost_ceiling_usd: Some(args.cost_ceiling),
        submit_api_url: args.submit_url,
        submit_api_key: args.submit_key,
    };

    config
}
