//! Telemetry CLI summary.
use codetether_telemetry::{GlobalTokenSnapshot, PersistentStats};

pub(super) fn print(persistent: &PersistentStats, token_snapshot: &GlobalTokenSnapshot) {
    println!("# CodeTether Telemetry\n");

    // Summary
    println!("## Summary\n");
    println!("**Tool Executions:** {}", persistent.summary());
    println!("**Token Usage:** {}", token_snapshot.summary());

    // Tool breakdown
    if !persistent.stats.executions_by_tool.is_empty() {
        println!("\n## Tool Breakdown\n");
        let mut tools: Vec<_> = persistent.stats.executions_by_tool.iter().collect();
        tools.sort_by(|a, b| b.1.cmp(a.1));
        for (tool, count) in tools.iter().take(10) {
            println!("- **{}**: {} executions", tool, count);
        }
    }

    // Files modified
    if !persistent.stats.files_modified.is_empty() {
        println!("\n## Files Modified\n");
        let mut files: Vec<_> = persistent.stats.files_modified.iter().collect();
        files.sort_by(|a, b| b.1.cmp(a.1));
        for (path, count) in files.iter().take(10) {
            println!("- **{}**: {} changes", path, count);
        }
    }
}
