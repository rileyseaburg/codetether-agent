//! Telemetry CLI tokens.
use codetether_cli_options::StatsArgs;
use codetether_telemetry::TOKEN_USAGE;

pub(super) fn print(args: &StatsArgs) {
    // Token usage by model
    if args.tokens || args.all {
        let model_snapshots = TOKEN_USAGE.model_snapshots();
        if !model_snapshots.is_empty() {
            println!("\n## Token Usage by Model\n");
            for snapshot in &model_snapshots {
                println!("- **{}**: {}", snapshot.name, snapshot.summary());
            }
        }
    }
}
