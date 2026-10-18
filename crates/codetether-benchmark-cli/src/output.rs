//! Benchmark text or JSON output.
use codetether_benchmark::BenchmarkSuiteResult;
pub(super) fn print(result: &BenchmarkSuiteResult, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("=== Benchmark Results ===\n");
        println!("Date: {}", result.run_date);
        println!("Agent: {} v{}\n", result.agent, result.agent_version);

        crate::models::print(result);
        crate::rankings::print(result);
    }
    Ok(())
}
