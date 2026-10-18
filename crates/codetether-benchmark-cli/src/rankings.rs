//! Winning model rankings from the benchmark result.
use codetether_benchmark::BenchmarkSuiteResult;
pub(super) fn print(result: &BenchmarkSuiteResult) {
    if !result.summary.rankings.is_empty() {
        println!("=== Rankings ===");
        println!("Best Pass Rate: {}", result.summary.best_pass_rate_model);
        println!("Fastest: {}", result.summary.fastest_model);
        println!("Cheapest: {}", result.summary.cheapest_model);
        println!("Best Overall: {}", result.summary.best_overall_model);
    }
}
