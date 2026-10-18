//! Telemetry CLI filtered.
use codetether_cli_options::StatsArgs;
use codetether_telemetry::PersistentStats;

pub(super) fn print(args: &StatsArgs, persistent: &PersistentStats) {
    // Filtered results
    if let Some(ref tool_name) = args.tool {
        let tool_execs = persistent.by_tool(tool_name);
        if !tool_execs.is_empty() {
            println!("\n## Executions of '{}'\n", tool_name);
            for exec in tool_execs.iter().take(args.limit) {
                let status = if exec.success { "✓" } else { "✗" };
                println!("- {} ({}ms)", status, exec.duration_ms);
            }
        }
    }

    if let Some(ref file_path) = args.file {
        let file_execs = persistent.by_file(file_path);
        if !file_execs.is_empty() {
            println!("\n## Changes to '{}'\n", file_path);
            for exec in file_execs.iter().take(args.limit) {
                println!("- **{}** ({}ms)", exec.tool_name, exec.duration_ms);
                for change in &exec.file_changes {
                    if change.path == *file_path
                        && let Some((start, end)) = change.line_range
                    {
                        println!("  Lines {}-{}", start, end);
                    }
                }
            }
        }
    }
}
