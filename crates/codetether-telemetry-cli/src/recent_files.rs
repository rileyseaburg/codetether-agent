//! Telemetry CLI recent files.
use codetether_cli_options::StatsArgs;
use codetether_telemetry::PersistentStats;

pub(super) fn print(args: &StatsArgs, persistent: &PersistentStats) {
    // Recent file changes
    if args.files || args.all {
        let changes = persistent.all_file_changes();
        if !changes.is_empty() {
            println!("\n## Recent File Changes\n");
            for (exec_id, change) in changes.iter().rev().take(args.limit) {
                println!("- [#{}] {}", exec_id, change.summary());
            }
        }
    }
}
