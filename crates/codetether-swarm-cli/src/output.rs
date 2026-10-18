//! Swarm result presentation.
use codetether_swarm_core::SwarmResult;
pub(super) fn print(result: &SwarmResult, json: bool) -> anyhow::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("=== Swarm Execution Results ===");
        println!(
            "Status: {}",
            if result.success { "SUCCESS" } else { "FAILED" }
        );
        println!("Subtasks: {}", result.subtask_results.len());
        println!("Speedup: {:.1}x", result.stats.speedup_factor);
        println!("Critical Path: {} steps", result.stats.critical_path_length);
        println!("\n{}", result.result);
    }
    Ok(())
}
