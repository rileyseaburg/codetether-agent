//! Per-model benchmark summaries.
use codetether_benchmark::BenchmarkSuiteResult;
pub(super) fn print(result: &BenchmarkSuiteResult) {
    for mr in &result.model_results {
        println!("--- {} ---", mr.model);
        println!(
            "  Pass Rate: {:.1}% ({}/{} stories)",
            mr.aggregate.overall_pass_rate * 100.0,
            mr.aggregate.total_stories_passed,
            mr.aggregate.total_stories
        );
        println!(
            "  Speed: {:.1} stories/hour ({:.1}s avg)",
            mr.aggregate.stories_per_hour, mr.aggregate.avg_seconds_per_story
        );
        println!(
            "  Cost: ${:.4}/story (${:.2} total)",
            mr.aggregate.avg_cost_per_story, mr.aggregate.total_cost_usd
        );
        println!(
            "  PRDs: {}/{} fully passed\n",
            mr.aggregate.prds_fully_passed, mr.aggregate.prds_attempted
        );
    }
}
