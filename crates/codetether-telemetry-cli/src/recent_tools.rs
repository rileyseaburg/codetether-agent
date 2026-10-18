//! Telemetry CLI recent tools.
use codetether_cli_options::StatsArgs;
use codetether_telemetry::PersistentStats;

pub(super) fn print(args: &StatsArgs, persistent: &PersistentStats) {
    // Recent executions
    if args.tools || args.all {
        let recent = persistent.recent(args.limit);
        if !recent.is_empty() {
            println!("\n## Recent Tool Executions\n");
            for exec in recent {
                let status = if exec.success { "✓" } else { "✗" };
                let files_str = if exec.file_changes.is_empty() {
                    String::new()
                } else {
                    format!(
                        " → {}",
                        exec.file_changes
                            .iter()
                            .map(|f: &codetether_telemetry::FileChange| f.summary())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                println!(
                    "- {} **{}** ({}ms){}",
                    status, exec.tool_name, exec.duration_ms, files_str
                );
            }
        }
    }
}
